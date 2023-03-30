# Kafka listener

![License: GPL-3.0](https://img.shields.io/github/license/InteractiveReduction/kafka-listener)
![Build: passing](https://img.shields.io/github/actions/workflow/status/interactivereduction/kafka-listener/tests.yml?branch=main)

## Running and testing

Expects multiple environment variables to be set:

- "KAFKA_IP" The kafka ip that this should listen to, with the relevant topics present
- "DB_IP" The IP for a PostgreSQL database that has the db schema applied
- "DB_USERNAME" The username for logging into the database
- "DB_PASSWORD" The password for logging into the database

Expected value on detected-runs kafka message example:
```
{"run_number": "123456", "instrument": "MARI", "experiment_title": "Super cool experiment", "run_start": "2023-03-28T07:15:50", "run_end": "2023-03-28T07:16:50", "experiment_number": "1220474", "users": "Dr Physics & co.", "filepath": "path/to/file/MARI0.nxs", "will_reduce": true, "good_frames": 1000000, "raw_frames": 1000001, "additional_values": { "additional_reduction_input_1": "I am an input!", "additional_reduction_input_2": "Me Too!" }}
```
Which as a JSON object looks like:
```json
{
  "run_number": "123456",
  "instrument": "MARI",
  "experiment_title": "Super cool experiment",
  "experiment_number": "1220474",
  "users": "Dr Physics & co.",
  "filepath": "path/to/file/MARI0.nxs",
  "will_reduce": true,
  "good_frames": 1000000,
  "raw_frames": 1000001,
  "run_start": "2023-03-28T07:15:50",
  "run_end": "2023-03-28T07:16:50",
  "additional_values": {
    "additional_reduction_input_1": "I am an input!",
    "additional_reduction_input_2": "Me Too!"
  }
}
```

## Building

Need libpq installed as a dependency:
Ubuntu
```bash
sudo apt-get install libpq-dev -y
```

TODO

## How to add migration

TODO

## How to container

TODO