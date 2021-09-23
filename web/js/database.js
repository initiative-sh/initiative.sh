import Dexie from "dexie";

const dexie = new Dexie("initiative");

dexie.version(1).stores({
  things: "&uuid, name, type",
});

const delete_thing_by_uuid = async (uuid) => {
  return dexie.things.delete(uuid)
    .then(() => true)
    .catch(() => false);
};

const get_all_the_things = async () => {
  return dexie.things.toArray()
    .catch(() => {});
};

const save_thing = async (thing) => {
  return dexie.things.put(thing)
    .then(() => true)
    .catch(() => false);
};

export { delete_thing_by_uuid, get_all_the_things, save_thing };
