import pandas as pd
import matplotlib.pyplot as plt

def plot_chart():
    try:
        # Wczytanie danych z pliku CSV
        df = pd.read_csv("data.csv", header=None, names=["Generation", "SuccessRate"])

        # Tworzenie wykresu
        plt.figure(figsize=(8, 6))
        plt.plot(df["Generation"], df["SuccessRate"], marker="o", linestyle="-", color="red", label="Procent sukcesu")
        plt.xlabel("Generacja")
        plt.ylabel("Procent rakiet, które dotarły do celu")
        plt.title("Ewolucja rakiet w czasie")
        plt.legend()
        plt.grid()

        # Zapis wykresu do pliku
        plt.savefig("chart.png")

    except Exception as e:
        print(f"Błąd podczas generowania wykresu: {e}")

if __name__ == "__main__":
    plot_chart()
