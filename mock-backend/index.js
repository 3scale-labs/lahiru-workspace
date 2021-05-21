const express = require('express')
const app = new express()
require('log-timestamp')

app.use(express.json());

app.get('/foo', (req,res) => {
    res.status(200).send({
        "message": "Hello from foo service !!!"
    })
})

app.get('/bar', (req,res) => {
    res.status(200).send({
        "message": "Hello from bar service !!!"
    })
})

app.get('/baz', (req,res) => {
    res.status(200).send({
        "message": "Hello from baz service !!!"
    })
})

app.listen(8000, ()=>{console.log("Backend service started listening on port: 8000")})