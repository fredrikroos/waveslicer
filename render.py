import sys
import waveslicer
import subprocess
import json

WIDTH = 800

mp3 = sys.argv[1]

print("Generating waveform...")
waveform = waveslicer.generate_waveform(mp3, WIDTH)
print("Generating JSON...")
waveform_json = json.dumps(waveform.data)

print("Creating HTML view...")
html = f"""
<html>
<head>
    <script src="../src/waveform.js"></script>
    <script>
       const data = {waveform_json}
    </script>
</head>
<body>
    <canvas id="wave" width="{WIDTH}" height="100" style="border:1px solid #000000;"></canvas>
    <script>
        const waveform = new Waveform();
        waveform.drawWaveform();
    </script>
</body>
</html>
"""

f = open("target/render.html", "w")
f.write(html)
print("Done!")
subprocess.call(["open", "target/render.html"])

