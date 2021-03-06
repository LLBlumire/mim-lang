# Mim (Programming Language)

Mim is a statically and strongly typed, implicitly memory managed (reference counting), functional, programming language that provides high level interfaces and powerful abstractions. It compiles to a format named MimB compatible with a lightweight virtual machine called Mimvm.

```mim
using Std::IO;
define Main {
    default mapping Main() {
        IO::PrintLine("Hello, World!");
    }
}
```

```mim
using Std::IO;
define Main {
    default mapping Main() {
        for n in Range::inclusive(1, 100) {
            select (n % 3, n % 5) {
                (true, true) => IO::PrintLine("Fizzbuzz"),
                (true, false) => IO::PrintLine("Fizz"),
                (false, true) => IO::PrintLine("Buzz"),
                (false, false) => IO::PrintLine(String::from(n))
            }
        }
    }
}
```

```mim
define GeoCoord {
    default product Coord {
        lat: f32,
        lon: f32,
        alt: f32
    }
    implement auto Copy for Coord;
    public mapping New(lat: f32, lon: f32, alt: f32): GeoCoord {
		GeoCoord::Angle {
			lat: lat,
			lon: lon,
			alt: alt
		}
	}
    public mapping Lat(self: &GeoCoord): &f32 {
	    &self.lat
    }
    public mapping Lon(self: &GeoCoord): &f32 {
	    &self.lon
    }
    public mapping Alt(self: &GeoCoord): &f32 {
	    &self.alt
    }
}
define GeoAngle {
	default product Angle {
		angle_from_north: f32,
	}
	public mapping New(angle_from_north: f32): GeoAngle {
		GeoAngle::Angle {
			angle_from_north: angle_from_north
		}
	}
}
define Vehicle {
    default protocol Vehicle {
	    type CoordinateSystem;
	    type OrientationSystem;
        mapping Position(self: &Implementor): CoordinateSystem;
        mapping Orientation(self: &Implementor): OrientationSystem;
        mapping TopSpeed(self: &Implementor): f32;
        mapping NumPassenger(self: &Implementor): u32;
        mapping NumWheels(self: &Implementor): u32;
        mapping AddPassenger(self: @Implementor): bool;
        mapping RemovePassenger(self: @Implementor): bool;
    }
}
define Car {
	default product Car {
		position: GeoCoord,
		angle: GeoAngle,
		make: String,
		model: String,
		top_speed: f32,
		passengers: u32,
		passenger_limit: u32
	}
	public mapping New(
		position: GeoCoord,
		angle: GeoAngle,
		make: String,
		model: String,
		top_speed: f32,
		passenger_limit: u32
	): Car {
		Car::Car {
			position: position,
			angle: angle,
			make: make,
			model: model,
			top_speed: top_speed,
			passengers: 0,
			passenger_limit: passenger_limit
		}
	}
	implement Vehicle for Car {
		type CoordinateSystem = GeoCood;
		type OrientationSystem = GeoAngle;
		mapping Position(self: &Car): CoordinateSystem {
			self.position
		}
		mapping Orientation(self: &Car): OrientationSystem {
			self.angle
		}
		mapping TopSpeed(self: &Car): f32 {
			self.top_speed
		}
		mapping NumPassenger(self: &Car): u32 {
			self.passengers
		}
		mapping NumWheels(self: &Car): u32 {
			4
		}
		mapping AddPassenger(self: @Car): bool {
			if this.passengers != this.passenger_limit {
				this.passengers += 1;
				true
			} else {
				false
			}
		}
		mapping RemovePassenger(self: &Car): bool {
			if this.passengers != 0 {
				this.passengers -= 1;
				true
			} else {
				false
			}
		}
	}
}
define Bike {
	default product Bike {
		position: GeoCoord,
		angle: GeoAngle,
		make: String,
		model: String,
		top_speed: f32,
		passengers: u32,
	}
	public mapping New(
		position: GeoCoord,
		angle: GeoAngle,
		make: String,
		model: String,
		top_speed: f32,
	): Bike {
		Bike::Bike {
			position: position,
			angle: angle,
			make: make,
			model: model,
			top_speed: top_speed,
			passengers: 0,
		}
	}
	implement Vehicle for Bike {
		type CoordinateSystem = GeoCood;
		type OrientationSystem = GeoAngle;
		mapping Position(self: &Car): CoordinateSystem {
			self.position
		}
		mapping Orientation(self: &Car): OrientationSystem {
			self.angle
		}
		mapping TopSpeed(self: &Car): f32 {
			self.top_speed
		}
		mapping NumPassenger(self: &Car): u32 {
			self.passengers
		}
		mapping NumWheels(self: &Car): u32 {
			2
		}
		mapping AddPassenger(self: @Car): bool {
			if this.passengers != 2 {
				this.passengers += 1;
				true
			} else {
				false
			}
		}
		mapping RemovePassenger(self: &Car): bool {
			if this.passengers != 0 {
				this.passengers -= 1;
				true
			} else {
				false
			}
		}
	}
}
define Main {
	default mapping Main() {
		let mut my_car = Car::New(
			GeoPosition::New(51.438041, -0.944441, 0.0),
			GeoAngle::New(0.0),
			"Ford",
			"Ka",
			159.5,
			4
		);
		let mut my_bike = Bike::new(
			GeoPosition::New(51.440178, -0.94303, 0.0),
			GeoAngle::New(3.14),
			"BMW",
			"S 1000 RR",
			299.0
		);
		
		AddPassenger(@my_bike);
		AddPassenger(@my_car);
		AddPassenger(@my_bike);
		AddPassenger(@my_bike); // Cannot Add!	
	}
	
	mapping AddPassenger<V: Vehicle>(car: @V) {
		if vehicle.@AddPassenger() {
			IO::PrintLine("Added Passenger");
		} else {
			IO::PrintLine("Failed to Add Passenger");
		}
	}
}

```
