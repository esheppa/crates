# Derivatives



## Marker traits

The job of these is to ensure that only sensible numeric operations are conducted
- It makes sense to multiply a price by a quantity, only if they have the same `Commodity`

`Scale` - eg Kilo, Mega, Giga

### `Unit` 
eg Barrel, Watt, Joule, Celsius. These are implemented by library consumers, however some are provided in examples and also default implementations

### `Currency` 
eg Aud, Usd, Jpy. The ISO4271 standard codes will be implemened in the library, however custom ones can also be added.

### `Observable` 
_a physical quantity that can be measured_
eg Wind speed in Melbourne, Temperature in Adelaide, GDP in Australia

(It would be nice to find a more generic word here to include things like GDP where it is somewhat tenuous to say that it is a physical quantity, although in many senses it is, however ability / instrumentation to measure is simply less effective that measurement of eg wind or temperature)

Could also call this `Process` like (stochastic process) in a sense that it is a thing that produces observations.

These are implemented by library consumers, however some are provided in examples and also default implementations.
They are made up of
- A `Unit`
- A standard `Scale`


### `Asset` 
These are implemented by library consumers, however some are provided in examples and also default implementations.

These are differentiated from `Observables`s in that `Asset`s can be traded and hence it makes sense for them to be a part of a `Price`, whereas `Observables` don't get traded and are simply points in a time series.

Other standard terminologies here are `Commodity` and `Product` however especially `Commodity` has connotations of fungibility wheras asset encompases a wider set of possible things of worth. `Product` also has connotations of tangibility whereas `Asset` has no such implication.

As an extra bonus, the abbreviation of `Asset` to `A` doesn't conflict with any of `Scale` (`S`), `Unit` (`U`), `Currency` (`C`) or `Observation` (`O`).

They are made up of
- A `Unit`
- A standard `Scale`
- A standard `Currency`
    - This is because when we ask a `PriceProvider` we should ask for prices of a `Commodity` only in its standard `Currency`

... location?

## Different kinds of values

### `Price`
Made up of a:
- `Commodity` 
- `Currency` 


### `Forex`

Made up of a:
- local `Currency` (fixed)
- remote `Currency` (varying)

This means that the actual value is denominated in the remote or varying `Currency`

for example:
- `Forex<AUD, USD>`: the amount of USD that can be purchased for 1 AUD
- `Forex<USD, AUD>`: the amount of AUD that can be purchased for 1 USD

### `Quantity` 
- amount of an asset


### `Observation`
- amount of an 


### `Amount`


