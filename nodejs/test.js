(async () => {
    const iot = require('aws-sdk/clients/iotdata')

    const client = new iot({ endpoint: "https://acwprzq1nws9h-ats.iot.eu-west-1.amazonaws.com", region: "eu-west-1" })

    if (process.env.NUM_TOPICS) {
        numTopics = parseInt(process.env.NUM_TOPICS)
        if (numTopics === NaN) {
            console.log(`Invalid NUM_TOPICS value ${process.env.NUM_TOPICS}`)
            process.exit(1)
        }
    } else {
        numTopics = 1000
    }

    const payload = Buffer.from("topic")
    console.log(`Publishing to ${numTopics} topics`)
    p = []
    let t = Date.now()
    for (i = 0; i < numTopics; i++) {
        topic = "test_" + i
        p.push(client.publish({ topic: topic, payload: payload }).promise())
    }

    await Promise.all(p)
    console.log(`Published in ${Date.now() - t}ms`)
})()
