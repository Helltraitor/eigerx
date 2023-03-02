from cheker import price_check


def test_from_first_example():
    product_names = ['eggs', 'milk', 'cheese']
    product_prices = [2.89, 3.29, 5.79]

    sold_names = ['eggs', 'eggs', 'cheese', 'milk']
    sold_prices = [2.89, 2.99, 5.97, 3.29]

    assert price_check(product_names, product_prices, sold_names, sold_prices) == 2


def test_from_second_example():
    product_names = ['rice', 'sugar', 'wheat', 'cheese']
    product_prices = [16.89, 56.92, 20.89, 345.99]

    sold_names = ['rice', 'cheese']
    sold_prices = [18.99, 400.89]

    assert price_check(product_names, product_prices, sold_names, sold_prices) == 2


if __name__ == "__main__":
    test_from_first_example()
    test_from_second_example()
