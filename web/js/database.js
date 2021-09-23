import Dexie from "dexie"

const dexie = new Dexie("initiative")

dexie.version(2).stores({
  things: "&uuid, name, type",
  keyValue: "&key",
})

const delete_thing_by_uuid = async (uuid) => {
  return dexie.things.delete(uuid)
    .then(() => true)
    .catch(() => false)
}

const get_all_the_things = async () => {
  return dexie.things.toArray()
    .catch(() => {})
}

const save_thing = async (thing) => {
  return dexie.things.put(thing)
    .then(() => true)
    .catch(() => false)
}

const set_value = async (key, value) => {
  return dexie.keyValue.put({key, value})
    .then(() => true)
    .catch(() => false)
}

const get_value = async (key) => {
  return dexie.keyValue.get(key)
    .then((result) => result.value)
    .catch(() => false)
}

const delete_value = async (key) => {
  return dexie.keyValue.delete(key)
    .then(() => true)
    .catch(() => false)
}

export {
  delete_thing_by_uuid,
  delete_value,
  get_all_the_things,
  get_value,
  save_thing,
  set_value,
}
