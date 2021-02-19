# mingo_rs
MongoDB Query and Aggregation logic in Rust for JSON matching and logical operations


The Mongo Query language allows great expressiveness and ease of use and is an excellent candidate for a general purpose JSON based general logic DSL.

Requirements:

- This project aims to duplicate the functionality of https://www.npmjs.com/package/mingo to allow for runtime-defined simple logic.
- This is NOT intended to interoperate with the MongoDB database in any way nor is it intended to support the volume or efficiency of MongoDB.
- It is entirely for leveraging a pre-defined DSL for runtime logic in cases where the overhead of ingesting data to MongoDB is greater than gains from doing the queries directly therein.
- It should be sufficient for testing of queries intended for processing in MongoDB during production.
- It should support matching against a single object with a boolean response or filtering of a collection of objects.


While this project is not currently in use in production anywhere, if maturity is reached, it will be used to speed up an existing automated flight rule checking system in use by JPL for the Mars2020 Perserverance Rover mission (currently written in Typescript running on NodeJS).
