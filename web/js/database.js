import Dexie from "dexie";

const dexie = new Dexie("initiative");

dexie.version(1).stores({
  things: "&uuid, name, type",
});

const save = async (thing) => {
  await dexie.things.put(thing);
}

const get_all = async () => await dexie.things.toArray();

export { save, get_all };
