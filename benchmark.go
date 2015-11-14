package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func main() {
	var benchmarkCount, err = strconv.Atoi(os.Args[1])
	if err != nil {
		panic("Please enter a valid benchmark")
	}

	results := make([]float64, benchmarkCount, benchmarkCount)
	c := make(chan float64)

	for i := 0; i < benchmarkCount; i++ {
		var start = time.Now().Nanosecond()
		go func() {
			var end = time.Now().Nanosecond()
			var sample float64 = float64(end-start) / 1000
			fmt.Printf("  sample %fms\n", sample)
			c <- sample
		}()
		results[i] = <-c
	}
	var total float64 = 0
	for _, sample := range results {
		total += sample / float64(benchmarkCount)
	}

	fmt.Printf("Average sample: %fms\n", total)
}
