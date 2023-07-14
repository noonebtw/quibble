#pragma once

#include <compare>
#include <cstddef>
#include <iterator>
#include <tuple>
#include <type_traits>
#include <utility>

namespace util {

template <int I, class... Ts>
decltype(auto) get(Ts&&... ts) {
  return std::get<I>(std::forward_as_tuple(ts...));
}

template <class Iterator>
class Range {
  Iterator m_begin, m_end;

 public:
  Range(const Iterator _begin, const Iterator _end)
      : m_begin(_begin), m_end(_end) {}

  Range(const std::pair<const Iterator, const Iterator>& t) {
    m_begin = t.first;
    m_end   = t.second;
  }

  template <typename... Args>
  Range(Args&&... args) : m_begin(get<0>(args...)), m_end(get<1>(args...)) {}

  auto begin() -> Iterator { return m_begin; }
  auto begin() const -> const Iterator { return m_begin; }

  auto end() -> Iterator { return m_end; }
  auto end() const -> const Iterator { return m_end; }
};

namespace detail {

template <class C>
requires requires(const C &c) { c.read(); }
using reference_t = decltype(std::declval<const C &>().read());

namespace detail {
template <class C> struct deduced_value_t {
  template <class T> static auto deduce(int) -> typename T::value_type;
  template <class T> static auto deduce(...) -> std::decay_t<reference_t<T>>;

  using type = decltype(deduce<C>(0));
};
} // namespace detail

template <class C>
requires std::same_as<typename detail::deduced_value_t<C>::type,
                      std::decay_t<typename detail::deduced_value_t<C>::type>>
using value_type_t = typename detail::deduced_value_t<C>::type;

namespace detail {
template <class C> struct deduced_difference_t {
  template <class T> static auto deduce(int) -> typename T::difference_type;
  template <class T>
  static auto deduce(long) -> decltype(std::declval<const T &>().distance_to(
      std::declval<const T &>()));
  template <class T> static auto deduce(...) -> std::ptrdiff_t;

  using type = decltype(deduce<C>(0));
};
} // namespace detail

template <class C>
using difference_type_t = typename detail::deduced_difference_t<C>::type;

template <typename I> struct iterator_traits {
  using difference_type = difference_type_t<I>;
  using value_type = value_type_t<I>;
  using reference = reference_t<I>;
};
} // namespace detail

template <typename I> struct iterator_t {
private:
  I base_;

public:
  iterator_t() requires(std::is_default_constructible_v<I>) : base_({}) {}
  iterator_t(const I &base) : base_(base) {}
  iterator_t(I &&base) : base_(std::move(base)) {}

  auto constexpr base() const -> const I & { return base_; }
  auto constexpr base() -> I & { return base_; }

  // input_or_output_iterator
  auto constexpr operator++(int) -> iterator_t
      requires(std::input_or_output_iterator<I>) {
    const auto ret = *this;
    base()++;
    return ret;
  }
  auto constexpr operator++()
      -> iterator_t &requires(std::input_or_output_iterator<I>) {
    this ++;
    return *this;
  }

  // input_iterator
  auto constexpr operator*() const -> const std::iter_reference_t<I>
  requires(std::input_iterator<I>) { return *base(); }

  // output_iterator
  template <typename T = decltype(*base_)>
  auto constexpr operator*() -> std::iter_reference_t<I>
  requires(std::output_iterator<I, T>) { return *base(); }

  // forward_iterator
  auto constexpr operator==(const iterator_t &rhs) const
      -> bool requires(std::forward_iterator<I>) {
    return base() == rhs.base();
  }

  template <std::sentinel_for<I> S>
  auto constexpr operator==(const S &rhs) const
      -> bool requires(std::forward_iterator<I>) {
    return base() == rhs;
  }

  // bidirectional_iterator
  auto constexpr operator--(int) -> iterator_t
      requires(std::bidirectional_iterator<I>) {
    const auto ret = *this;
    base()--;
    return ret;
  }
  auto constexpr operator--()
      -> iterator_t &requires(std::bidirectional_iterator<I>) {
    base()--;
    return *this;
  }

  // random_access_iterator
  auto constexpr operator<=>(const iterator_t &rhs) const
      -> std::strong_ordering {
    return base() <=> rhs.base();
  }

  auto constexpr operator-(const iterator_t &rhs) const
      -> std::iter_difference_t<I> {
    return base() - rhs.base();
  }
  auto constexpr operator+(const std::iter_difference_t<I> &n) const
      -> iterator_t {
    return iterator_t(base() + n);
  }
  friend auto constexpr operator+(const std::iter_difference_t<I> &n,
                                  const iterator_t &rhs) -> iterator_t {
    return rhs + n;
  }
  auto constexpr operator-(const std::iter_difference_t<I> &n) const
      -> iterator_t {
    return iterator_t(base() - n);
  }

  auto operator+=(const std::iter_difference_t<I> &n) -> iterator_t & {
    *this = iterator_t(base() + n);
    return *this;
  }
  auto operator-=(const std::iter_difference_t<I> &n) -> iterator_t & {
    *this = iterator_t(base() - n);
    return *this;
  }

  auto constexpr operator[](const std::iter_difference_t<I> &n) const
      -> const std::iter_reference_t<I> {
    return base()[n];
  }
  auto operator[](const std::iter_difference_t<I> &n)
      -> std::iter_reference_t<I> {
    return base()[n];
  }
};

template <typename T, typename I>
concept iterator = std::same_as<typename iterator_t<I>::value_type, T>;

}  // namespace util

namespace std {
template <typename I>
struct iterator_traits<util::iterator_t<I>> : public iterator_traits<I> {};
} // namespace std

#include <ranges>

namespace util {
namespace rg = std::ranges;

template <class R>
concept simple_view = // exposition only
    rg::view<R> && rg::range<const R> &&
    std::same_as<std::ranges::iterator_t<R>,
                 std::ranges::iterator_t<const R>> &&
    std::same_as<std::ranges::sentinel_t<R>, std::ranges::sentinel_t<const R>>;

template <rg::input_range R>
requires(rg::view<R>) class enumerate_view
    : public rg::view_interface<enumerate_view<R>> {
private:
  R base_ = R();

public:
  template <bool Const> class iterator;
  template <bool Const> class sentinel;

  constexpr enumerate_view() = default;
  constexpr enumerate_view(R base) : base_(std::move(base)) {}

  constexpr auto size() -> rg::range_difference_t<R>
  requires(rg::sized_range<R>) { return rg::size(base_); }
  constexpr auto size() const -> rg::range_difference_t<R>
  requires(rg::sized_range<const R>) { return rg::size(base_); }

  constexpr auto begin() -> iterator<false> {
    return iterator<false>{rg::begin(base_), 0};
  }

  constexpr auto begin() const -> iterator<true> {
    return iterator<true>{rg::begin(base_), 0};
  }

  constexpr auto end() { return sentinel<false>(rg::end(base_)); }

  constexpr auto end() requires(rg::common_range<R> &&rg::sized_range<R>) {
    return iterator<false>{rg::end(base_),
                           static_cast<rg::range_difference_t<R>>(size())};
  }

  constexpr auto end() const requires(rg::range<const R>) {
    return sentinel<true>{rg::end(base_)};
  }

  constexpr auto end() const
      requires(rg::common_range<const R> &&rg::sized_range<R>) {
    return iterator<true>{rg::end(base_),
                          static_cast<rg::range_difference_t<R>>(size())};
  }

  constexpr auto base() const -> R &requires(std::copy_constructible<R>) {
    return base_;
  }
  constexpr auto base() -> R && { return std::move(base_); }
};

template <class R> enumerate_view(R &&) -> enumerate_view<std::views::all_t<R>>;

template <rg::input_range R>
requires(rg::view<R>) template <bool Const> class enumerate_view<R>::iterator {
public:
  using Base = std::conditional_t<Const, const R, R>;
  using iterator_type = rg::iterator_t<Base>;
  using iterator_category =
      typename std::iterator_traits<iterator_type>::iterator_category;
  using iterator_concept = iterator_category;

  using difference_type = typename rg::range_difference_t<Base>;
  using enumerator_type = difference_type;

  struct result {
    const enumerator_type index;
    rg::range_reference_t<Base> value;
  };

  using reference = result;
  using value_type = result;

public:
  enumerator_type index_ = {};
  iterator_type base_ = iterator_type();

  iterator() = default;
  constexpr explicit iterator(iterator_type base, enumerator_type index = 0)
      : base_(std::move(base)), index_(index) {}
  constexpr iterator(iterator<!Const> i) requires Const
      && std::convertible_to<rg::iterator_t<R>, rg::iterator_t<Base>>
      : base_(std::move(i.base_)), index_(std::move(i.index_)) {}

  constexpr rg::iterator_t<Base>
  base() const &requires std::copyable<rg::iterator_t<Base>> {
    return base_;
  }
  constexpr rg::iterator_t<Base> base() && { return std::move(base_); }

  constexpr decltype(auto) operator*() const { return result{index_, *base_}; }

  constexpr auto operator++() -> iterator & {
    ++index_;
    ++base_;
    return *this;
  }

  constexpr auto operator++(int) -> void requires(!rg::forward_range<Base>) {
    ++index_;
    ++base_;
  }

  constexpr auto operator++(int) -> iterator requires(rg::forward_range<Base>) {
    const auto tmp = *this;
    ++index_;
    ++base_;
    return tmp;
  }

  constexpr auto operator--()
      -> iterator &requires(rg::bidirectional_range<Base>) {
    --index_;
    --base_;
    return *this;
  }

  constexpr auto operator--(int) -> iterator
      requires(rg::bidirectional_range<Base>) {
    const auto tmp = *this;
    --index_;
    --base_;
    return tmp;
  }

  constexpr auto operator+=(difference_type n)
      -> iterator &requires(rg::random_access_range<Base>) {
    index_ += n;
    base_ += n;
    return *this;
  }

  constexpr auto operator-=(difference_type n)
      -> iterator &requires(rg::random_access_range<Base>) {
    index_ -= n;
    base_ -= n;
    return *this;
  }

  friend constexpr auto operator==(const iterator &lhs, const iterator &rhs)
      -> bool requires(std::equality_comparable<rg::iterator_t<Base>>) {
    return lhs.base_ == rhs.base_;
  }

  friend constexpr auto operator<(const iterator &lhs, const iterator &rhs)
      -> bool requires(rg::random_access_range<Base>) {
    return lhs.base_ < rhs.base_;
  }
  friend constexpr auto operator>(const iterator &lhs, const iterator &rhs)
      -> bool requires(rg::random_access_range<Base>) {
    return rhs < lhs;
  }
  friend constexpr auto operator<=(const iterator &lhs, const iterator &rhs)
      -> bool requires(rg::random_access_range<Base>) {
    return !(rhs < lhs);
  }
  friend constexpr auto operator>=(const iterator &lhs, const iterator &rhs)
      -> bool requires(rg::random_access_range<Base>) {
    return !(lhs < rhs);
  }

  friend constexpr auto operator<=>(
      const iterator &lhs,
      const iterator &rhs) requires(std::three_way_comparable<iterator_type>) {
    return lhs.base_ <=> rhs.base_;
  }

  friend constexpr auto operator+(const iterator &lhs, difference_type n)
      -> iterator requires(rg::random_access_range<Base>) {
    return iterator{lhs} + n;
  }

  friend constexpr auto operator+(difference_type n, const iterator &rhs)
      -> iterator requires(rg::random_access_range<Base>) {
    return n + rhs;
  }

  friend constexpr auto operator-(const iterator &lhs, difference_type n)
      -> iterator requires(rg::random_access_range<Base>) {
    return iterator{lhs} -= n;
  }

  friend constexpr auto operator-(const iterator &lhs, const iterator &rhs)
      -> iterator requires(rg::random_access_range<Base>) {
    return lhs.base_ - rhs.base_;
  }
};

template <rg::input_range R>
requires(rg::view<R>) template <bool Const> class enumerate_view<R>::sentinel {
private:
  using Base = std::conditional_t<Const, const R, R>;
  rg::sentinel_t<Base> end_ = rg::sentinel_t<Base>();

public:
  sentinel() = default;
  constexpr explicit sentinel(rg::sentinel_t<Base> end) : end_(end) {}
  constexpr sentinel(sentinel<!Const> other) requires Const
      && std::convertible_to<rg::sentinel_t<R>, rg::sentinel_t<Base>>
      : end_(std::move(other.end_)) {}

  constexpr auto base() const -> rg::sentinel_t<Base> { return end_; }

  friend constexpr auto operator==(const iterator<Const> &rhs,
                                   const sentinel &lhs) -> bool {
    return rhs.base_ == lhs.end_;
  }

  friend constexpr auto operator-(const iterator<Const> &rhs,
                                  const sentinel &lhs)
      -> rg::range_difference_t<Base> {
    return rhs.base_ - lhs.end_;
  }
  friend constexpr auto operator-(const sentinel &rhs,
                                  const iterator<Const> &lhs)
      -> rg::range_difference_t<Base> {
    return rhs.end_ - lhs.base_;
  }
};

namespace detail {

struct enumerate_view_adaptor {
  template <typename R> constexpr auto operator()(R &&r) const {
    return enumerate_view{std::move(r)};
  }

  template <rg::range R>
  constexpr friend auto operator|(R &&rng, const enumerate_view_adaptor &) {
    return enumerate_view{std::move(rng)};
  }
};
} // namespace detail

namespace views {
inline detail::enumerate_view_adaptor enumerate;
} // namespace views

} // namespace util
