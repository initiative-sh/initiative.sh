import Dexie from "dexie"

const dexie = new Dexie("initiative")

dexie.version(6).stores({
  things: "&uuid, &name, type",
  keyValue: "&key",
}).upgrade((tx) => {
  return tx.table("things").toCollection().modify((thing) => {
    if (thing.type === "Location") {
      if (thing.subtype && thing.subtype.subtype) {
        thing.subtype = thing.subtype.subtype
      }
    }
  })
})

dexie.version(5).stores({
  things: "&uuid, &name, type",
  keyValue: "&key",
})

dexie.version(4).stores({
  things: "&uuid, name, type",
  keyValue: "&key",
}).upgrade((tx) => {
  return tx.table("things").toCollection().modify((thing) => {
    if (thing.type === "Npc") {
      if (thing.age && thing.age.value) {
        thing.age_years = thing.age.value
      }

      if (thing.age && thing.age.type) {
        thing.age = thing.age.type
      }
    }
  })
})

dexie.version(3).stores({
  things: "&uuid, name, type",
  keyValue: "&key",
}).upgrade((tx) => {
  return tx.table("things").toCollection().modify((thing) => {
    if (thing.type === "Npc") {
      if (thing.gender == "Trans") {
        thing.gender = "NonBinaryThey"
      }
    }
  })
})

dexie.version(2).stores({
  things: "&uuid, name, type",
  keyValue: "&key",
})

dexie.version(1).stores({
  things: "&uuid, name, type",
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
