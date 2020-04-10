<template>
  <div>
    <canvas ref="fpsGraph" width="400" height="300"></canvas>
  </div>
</template>

<script>
import Chart from "chart.js";

export default {
  data() {
    return {
      files: [],
      chart: null,
      chartdata: {
        labels: Array.from({ length: 300 }, (v, i) => i),
        datasets: []
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        animation: {
          duration: 0
        },
        hover: {
          animationDuration: 0
        },
        responsiveAnimationDuration: 0
      }
    };
  },

  mounted() {
    this.chart = new Chart(this.$refs.fpsGraph, {
      type: "line",
      data: this.chartdata,
      options: this.options
    });
    this.loadData();
  },

  methods: {
    async fetchFiles() {
      const res = await fetch("/ict40010/perf_index.json");
      const data = await res.json();
      this.files = data.files;
    },
    async loadData() {
      await this.fetchFiles();
      await Promise.all(
        this.files.map(async file => {
          const res = await fetch(`/ict40010/perfs/${file}`);
          const data = await res.json();
          this.chartdata.datasets.push({
            label: file,
            data: data.fps
          });
        })
      );
      this.chart.update();
    }
  }
};
</script>
