from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
import time
import subprocess

class DataHandler(FileSystemEventHandler):
    def on_modified(self, event):
        if event.src_path.endswith("data.csv"):  # Sprawdzamy czy zmienił się plik CSV
            print("Dane zmienione, generowanie nowego wykresu...")
            subprocess.run(["python", "plot.py"])

observer = Observer()
observer.schedule(DataHandler(), path=".", recursive=False)
observer.start()

print("Obserwowanie pliku data.csv...")

try:
    while True:
        time.sleep(1)
except KeyboardInterrupt:
    observer.stop()

observer.join()
