import Dexie from "dexie";

const dexie = new Dexie("initiative");

dexie.version(1).stores({
  things: "&uuid, name, type",
});

const get_all = async () => {
  return dexie.things.toArray()
    .catch(() => {});
};

const save = async (thing) => {
  return dexie.things.put(thing)
    .then(() => true)
    .catch(() => false);
};

export { get_all, save };
