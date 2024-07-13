import minepy
import numpy as np

def test_mic1():
    x = [1, 2, 3]
    y = [4, 5, 6]
    mic1 = minepy.compute_mic_list(x, y)
    print(mic1)
    m = minepy.Mine()
    m.compute_score_list(x, y)
    mic2 = m.mic()
    print(mic2)
    assert abs(mic1 - mic2) < 1e-6

def test_mic2():
    x = np.linspace(0, 1, 100)
    y = np.sin(x * 2 * np.pi)
    mic = minepy.compute_mic_numpy(x, y)
    print(mic)
    assert abs(mic - 1.0) < 1e-6

if __name__ == "__main__":
    test_mic1()
    test_mic2()
