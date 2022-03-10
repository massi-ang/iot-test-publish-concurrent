package main

import (
	"context"
	"fmt"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/iotdataplane"
	"log"
	"os"
	"strconv"
	"sync"
	"time"
)

func main() {
	numTopicsEnv, pres := os.LookupEnv("NUM_TOPICS")
	if !pres {
		numTopicsEnv = "1000"
	}
	numTopics, err := strconv.Atoi(numTopicsEnv)
	if err != nil {
		log.Fatalf("Invalid NUM_TOPICS value \"%s\"\n", numTopicsEnv)
	}
	// Load the Shared AWS Configuration (~/.aws/config)
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		log.Fatal(err)
	}
	endpointResolver := iotdataplane.EndpointResolverFromURL("https://acwprzq1nws9h-ats.iot.eu-west-1.amazonaws.com")

	// Create an Amazon S3 service client
	client := iotdataplane.NewFromConfig(cfg, iotdataplane.WithEndpointResolver(endpointResolver))
	topic := "test0"
	start := time.Now()
	_, err = client.Publish(context.TODO(), &iotdataplane.PublishInput{
		Topic:   &topic,
		Payload: []byte(topic),
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Single publish in %f\n", time.Since(start).Seconds())
	var wg sync.WaitGroup

	start = time.Now()
	payload := []byte(fmt.Sprintf("test %d", time.Now()))
	for i := 0; i < numTopics; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			topic := fmt.Sprintf("test_%d", i)
			_, err = client.Publish(context.TODO(), &iotdataplane.PublishInput{
				Topic:   &topic,
				Payload: payload,
				Qos:     0,
			})
			if err != nil {
				log.Println(err)
			}
		}(i)
	}
	wg.Wait()
	fmt.Printf("published in %f\n", time.Since(start).Seconds())
}
