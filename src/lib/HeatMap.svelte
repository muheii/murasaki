<script lang="ts">
	import { select, scaleThreshold } from 'd3';
    import { fullYearData } from './stores/stats-state.svelte';
	import { onMount } from 'svelte';

    function renderHeatmap() {
        if (!$fullYearData) { return; }
        const data = $fullYearData.daily_activity;
        
        const cellSize = 16;
        const cellMargin = 3;
        const fullCellSize = cellSize + cellMargin;
        const weekCount = 53;
        const dayCount = 7;
        
        const today = new Date();
        const currentYear = today.getFullYear();
        const startDate = new Date(currentYear, 0, 1);
        
        // Calculate first day of week offset
        const firstDayOfWeek = startDate.getDay();
        
        const width = weekCount * fullCellSize;
        const height = dayCount * fullCellSize + 30;
        
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
        let week = 0;
        
        while (currentDate.getFullYear() === currentYear) {
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
                displayDate: currentDate.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
            });
            
            currentDate.setDate(currentDate.getDate() + 1);
        }
        
        const svg = select('#heatmap-container')
            .append('svg')
            .attr('height', height)
            .attr('width', '100%')
            .attr('viewBox', `0 0 ${width} ${height}`)
            .attr('preserveAspectRatio', 'xMidYMid meet')
            .attr('class', "contribution-graph");
        
        const colorScale = scaleThreshold<number, string>()
            .domain([1, 30, 60, 120, 240])
            .range([
                "#252029", // 0 minutes
                "#9d4edd", // 1-29 minutes
                "#7b2cbf", // 30-59 minutes
                "#5a189a", // 60-119 minutes
                "#3c096c", // 120-239 minutes
                "#240046"  // 240+ minutes
            ]);
        
        const dayLabels = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
        svg.selectAll(".day-label")
            .data(dayLabels)
            .enter()
            .append("text")
            .attr("class", "day-label")
            .attr("y", (d, i) => i * fullCellSize + fullCellSize / 2 + 4)
            .attr("x", 0)
            .attr("text-anchor", "start")
            .attr("font-size", "11px")
            .attr("fill", "#a9a6ae")
            .attr("font-weight", "bold") 
            .text(d => d);
            
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
            .attr("x", d => (d.week * fullCellSize) + 30)
            .attr("y", d => d.day * fullCellSize)
            .attr("rx", 4)
            .attr("ry", 4)
            .attr("fill", d => {
                if (d.isEmpty) return "transparent";
                return colorScale(d.total_minutes);
            })
            .attr("stroke", "#352f3d")
            .attr("stroke-width", 0.5)
            .filter(d => !d.isEmpty)
            .append("title") 
            .text(d => {
                const minutes = d.total_minutes;
                const hours = Math.floor(minutes / 60);
                const remainingMins = minutes % 60;
                const timeString = hours > 0 ? 
                    `${hours}h ${remainingMins}m` : 
                    `${remainingMins}m`;
                
                return `${d.displayDate}: ${timeString}`;
            });
        
        const legendItems = [
            { label: "0m", color: "#252029" },
            { label: "<30m", color: "#9d4edd" },
            { label: "<60m", color: "#7b2cbf" },
            { label: "<2h", color: "#5a189a" },
            { label: "<4h", color: "#3c096c" },
            { label: ">4h", color: "#240046" }
        ];
        
        const legend = svg.append("g")
            .attr("transform", `translate(${width - 280}, ${height - 25})`);
        
        legend.selectAll(".legend-item")
            .data(legendItems)
            .enter()
            .append("rect")
            .attr("class", "legend-item")
            .attr("width", 12)
            .attr("height", 12)
            .attr("x", (d, i) => i * 50)
            .attr("y", 0)
            .attr("fill", d => d.color)
            .attr("rx", 3)
            .attr("ry", 3);
        
        legend.selectAll(".legend-label")
            .data(legendItems)
            .enter()
            .append("text")
            .attr("class", "legend-label")
            .attr("x", (d, i) => i * 50 + 16)
            .attr("y", 10)
            .attr("font-size", "10px")
            .attr("fill", "#a9a6ae")
            .text(d => d.label);
    }

    onMount(() => {
        renderHeatmap();
    });
</script>

<div class="bg-card border border-border rounded-xl shadow-md mb-4">
    <div id="heatmap-container" class="overflow-x-auto pt-10 pb-4"></div>
</div>