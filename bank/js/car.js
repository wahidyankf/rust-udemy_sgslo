const engine = {
  working: true,
};

const mustang = {
  name: "Mustang",
  engine: engine,
};

const camaro = {
  name: "Camaro",
  engine: engine,
};

function checkEngine(car) {
  if (car.name === "Mustang") {
    car.engine.working = false;
  }
}

console.log(mustang);
console.log(camaro);
console.log(checkEngine(camaro));
console.log(checkEngine(mustang));
