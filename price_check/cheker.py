def price_check(
    product_names: list[str],
    product_prices: list[float],
    sold_names: list[str],
    sold_prices: list[float],
) -> int:
    """
    As I assume this is the fastest solution. Collects product names and prices
    in a dict which reduces iteration for O(n) where n is len of sold names

    Example:
        >>> product_names = ['eggs', 'milk', 'cheese']
        >>> product_prices = [2.89, 3.29, 5.79]
        >>>
        >>> sold_names = ['eggs', 'eggs', 'cheese', 'milk']
        >>> sold_prices = [2.89, 2.99, 5.97, 3.29]
        >>>
        >>> assert price_check(product_names, product_prices, sold_names, sold_prices) == 2

    Args:
        product_names: List of product names
        product_prices: List of product prices
        sold_names: List of sold product names
        sold_prices: List of sold product prices

    Returns:
        Positive integer - number of differences between sold and real price

    Raises:
        ValueError when product or sold names have different size from its prices
        KeyError when sold name is not contained by product names list
    """
    if len(product_names) != len(product_prices):
        raise ValueError("Product names and prices must have same size")

    if len(sold_names) != len(sold_prices):
        raise ValueError("Sold names and prices must have same size")

    # Differences counter
    diffs = 0

    # Use dict instead of double for cycle
    products = dict(zip(product_names, product_prices))
    for index in range(len(sold_names)):
        # Notes:
        #       1. Product will throw key error if name is not contained by product names
        #       2. Float == Float is not good solution, probably better to use math.isclose
        diffs += products[sold_names[index]] != sold_prices[index]

    return diffs
