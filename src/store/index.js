import { createStore } from "vuex";

export default createStore({
  strict: process.env.NODE_ENV !== "production",
  state: {
    variables: {
      count: 50,
      velocity: {
        x: 1,
        y: 0,
        z: 0,
      },
      position: {
        x: 0,
        y: 0,
        z: 1,
      },
      mass: 1,
      charge: 1,
      intensity: 1,
      timeMs: 100,
    },
    three: {},
  },
  mutations: {
    updateVariables(state, payload) {
      state.variables = {
        ...state.variables,
        ...payload,
      };
    },
    setCount(state, payload) {
      if (typeof payload === "string") payload = parseFloat(payload);
      if (typeof payload === "number" || typeof payload === "bigint") {
        state.variables.count = payload;
      }
      // TODO Bigint, ¿function?
    },
    setVelocity(state, payload) {
      if ("x" in payload && "y" in payload && "z" in payload) {
        state.variables.velocity = payload;
      }
    },
    setPosition(state, payload) {
      if ("x" in payload && "y" in payload && "z" in payload) {
        state.variables.position = payload;
      }
    },
    setMass(state, payload) {
      if (typeof payload === "string") payload = parseFloat(payload);
      if (typeof payload === "number" || typeof payload === "bigint") {
        state.variables.mass = payload;
      }
      // TODO Bigint, ¿function?
    },
    setCharge(state, payload) {
      if (typeof payload === "string") payload = parseFloat(payload);
      if (typeof payload === "number" || typeof payload === "bigint") {
        state.variables.mass = payload;
      }
      // TODO Bigint, ¿function?
    },
    setIntensity(state, payload) {
      if (typeof payload === "string") payload = parseFloat(payload);
      if (typeof payload === "number" || typeof payload === "bigint") {
        state.variables.mass = payload;
      }
      // TODO Bigint, ¿function?
    },
    setTimeMs(state, payload) {
      if (typeof payload === "string") payload = parseFloat(payload);
      if (typeof payload === "number" || typeof payload === "bigint") {
        state.variables.mass = payload;
      }
      // TODO Bigint, ¿function?
    },
  },
  actions: {},
  modules: {},
});
