import Dexie from "dexie";

const dexie = new Dexie("initiative");

dexie.version(1).stores({
  things: "&uuid, name, type",
});

const delete_by_uuid = async (uuid) => {
  return dexie.things.delete(uuid)
    .then(() => true)
    .catch(() => false);
};

const get_all = async () => {
  return dexie.things.toArray()
    .catch(() => {});
};

const save = async (thing) => {
  return dexie.things.put(thing)
    .then(() => true)
    .catch(() => false);
};

export { delete_by_uuid, get_all, save };
