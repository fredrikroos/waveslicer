const MAX_ABS_SAMPLE_VALUE = 128;
class Waveform {
    constructor() {
        this.samples = data;

        this.waveform_canvas = document.getElementById("wave");
        this.width = this.waveform_canvas.width;
        this.height = this.waveform_canvas.height;
    }

    getNormalizationFactor() {
        return this.height / (2 * MAX_ABS_SAMPLE_VALUE);
    }

    drawWaveform() {
        var cxt = this.waveform_canvas.getContext('2d');
        cxt.globalCompositeOperation = 'source-over';

        // Colors
        cxt.strokeStyle = 'rgb(65,0,0)';
        cxt.fillStyle = 'rgb(195,0,0)';

        // Draw middle line
        cxt.beginPath();
        cxt.moveTo(0, this.height / 2);
        cxt.lineTo(this.width, this.height / 2);
        cxt.stroke();

        // Constants
        var pos = 0;
        var step = 1;
        var normalization_factor = -1 * this.getNormalizationFactor();
        cxt.beginPath();
        cxt.moveTo(0, this.height / 2);

        // Path through the lower bound
        for (let i = 0; i < this.samples.length; i += 2 * step) {
            pos += step;
            cxt.lineTo(pos, this.height / 2 + normalization_factor * this.samples[i]);
        }

        // Path through the upper bound
        for (let i = this.samples.length - 1; i > 0; i -= 2 * step) {
            pos -= step;
            cxt.lineTo(pos, this.height / 2 + normalization_factor * this.samples[i]);
        }

        // Draw waveform border
        cxt.stroke();
        // Fill waveform
        cxt.fill();
    }
}