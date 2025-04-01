const engine = {
  working: true,
};

const mustang = {
  name: 'Mustang',
  engine,
};

const camaro = {
  name: 'Camaro',
  engine,
};

function checkEngine(car) {
  if (car.name === 'Mustang') {
    car.engine.working = false;
  }
}
