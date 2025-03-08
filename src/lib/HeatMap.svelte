<script lang="ts">
	import { select, scaleThreshold } from 'd3';
    import { fullYearData } from './stores/stats-state.svelte';
	import { onMount } from 'svelte';

    function renderHeatmap() {
        if (!$fullYearData) { return; }
        const data = $fullYearData.daily_activity;
        const cellSize = 9;
        const cellMargin = 2;
        const fullCellSize = cellSize + cellMargin;
        const dayCount = 7;
        
        const today = new Date();
        const currentYear = today.getFullYear();
        const startDate = new Date(currentYear, 0, 1);
        const endDate = new Date(currentYear, 11, 31);
        
        const firstDayOfWeek = startDate.getDay();
        const lastDayOfWeek = endDate.getDay();
        
        const daysInYear = (endDate.getTime() - startDate.getTime()) / (24 * 60 * 60 * 1000) + 1;
        
        const weekCount = Math.ceil((firstDayOfWeek + daysInYear) / 7);
        
        const width = dayCount * fullCellSize;
        const height = weekCount * fullCellSize;
        
        const activityMap = new Map();
        data.forEach(d => {
            activityMap.set(d.date, d.total_minutes);
        });
        
        const dateArray = [];
        
        for (let i = 0; i < firstDayOfWeek; i++) {
            dateArray.push({
                date: "",
                total_minutes: 0,
                day: i,
                week: 0,
                isEmpty: true,
            });
        }
        
        const currentDate = new Date(startDate);
        const realToday = new Date();
        let week = 0;
        
        while (currentDate <= endDate) {
            const day = currentDate.getDay();
            
            if (day === 0 && dateArray.length > 0) {
                week++;
            }
            
            const dateStr = currentDate.toISOString().split('T')[0];
            
            dateArray.push({
                date: dateStr,
                total_minutes: activityMap.get(dateStr) || 0,
                day: day,
                week: week,
                isEmpty: false,
            });
            
            currentDate.setDate(currentDate.getDate() + 1);
        }
        
        const svg = select('#heatmap-container')
            .append('svg')
            .attr('height', height)
            .attr('width', width)
            .attr('class', "contribution-graph");
            
        const colorScale = scaleThreshold<number, string>()
            .domain([1, 30, 60, 120, 240])
            .range([
                "#ebedf0", // 0 minutes 
                "#9be9a8", // 1-29 minutes
                "#40c463", // 30-59 minutes
                "#30a14e", // 60-119 minutes
                "#216e39", // 120-239 minutes
                "#0d4429"  // 240+ minutes
            ]);
        
        svg.selectAll(".day")
            .data(dateArray)
            .enter()
            .append("rect")
            .attr("class", d => {
                if (d.isEmpty) return "day empty";
                return "day";
            })
            .attr("width", cellSize)
            .attr("height", cellSize)
            .attr("x", d => d.day * fullCellSize)
            .attr("y", d => d.week * fullCellSize)
            .attr("fill", d => {
                if (d.isEmpty) return "transparent";
                return colorScale(d.total_minutes);
            })
            .attr("rx", 0) 
            .attr("ry", 0)
            .filter(d => !d.isEmpty)
            .append("title") 
            .text(d => {
                return `${d.date}: ${d.total_minutes} minutes`;
            });
    }

    onMount(() => {
        renderHeatmap();
    })
</script>

<div id="heatmap-container"></div>