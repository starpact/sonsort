from random import randint


def bubble_sort(arr):
    for i in range(len(arr)):
        swapped = False
        for j in range(1, len(arr) - i):
            if arr[j - 1] > arr[j]:
                arr[j - 1], arr[j] = arr[j], arr[j - 1]
                swapped = True
        if not swapped:
            break


def selection_sort(arr):
    for i in range(len(arr) - 1):
        min_index = i
        for j in range(i, len(arr)):
            if arr[j] < arr[min_index]:
                min_index = j
        if i != min_index:
            arr[i], arr[min_index] = arr[min_index], arr[i]


def insertion_sort(arr):
    for unsorted in range(1, len(arr)):
        i = unsorted
        while i > 0 and arr[i] < arr[i - 1]:
            arr[i - 1], arr[i] = arr[i], arr[i - 1]
            i -= 1


def quick_sort(arr):
    def _quick_sort(start, end):
        if end - start <= 1:
            return

        pivot = arr[start]
        left, right = start + 1, end - 1
        while left < right:
            if arr[left] <= pivot:
                left += 1
            elif arr[right] >= pivot:
                right -= 1
            else:
                arr[left], arr[right] = arr[right], arr[left]
                left += 1
                right -= 1
        if arr[left] > pivot:
            left -= 1
        arr[start], arr[left] = arr[left], arr[start]
        _quick_sort(start, left)
        _quick_sort(left + 1, end)

    _quick_sort(0, len(arr))


def merge_sort(arr):
    helper = [0 for _ in range(len(arr))]

    def f(start, end):
        if end - start <= 1:
            return
        mid = (start + end) >> 1
        f(start, mid)
        f(mid, end)
        i, j, index = start, mid, start
        while True:
            if i == mid:
                break
            if j == end:
                arr[index:end] = arr[i:mid]
                break
            if arr[i] < arr[j]:
                helper[index] = arr[i]
                i += 1
            else:
                helper[index] = arr[j]
                j += 1
            index += 1
        arr[start:index] = helper[start:index]

    f(0, len(arr))


def test_sort(sort):
    ARRAY_LEN = 10
    arr = [randint(0, 10000) for _ in range(ARRAY_LEN)]
    arr_correct = arr.copy()
    arr_correct.sort()
    sort(arr)
    assert arr == arr_correct


if __name__ == "__main__":
    test_sort(bubble_sort)
    test_sort(selection_sort)
    test_sort(insertion_sort)
    test_sort(quick_sort)
    test_sort(merge_sort)
