import Dexie from "dexie"

const dexie = new Dexie("initiative")

dexie.version(7).stores({
  things: "&uuid, &name, type",
  keyValue: "&key",
}).upgrade((tx) => {
  return tx.table("things").toCollection().modify((thing) => {
    switch (thing.age) {
      case "YoungAdult":
        thing.age = "young-adult"
        break
      case "MiddleAged":
        thing.age = "middle-aged"
        break
      default:
        if (thing.age) {
          thing.age = thing.age.toLowerCase()
        }
    }

    if (thing.ethnicity) {
      thing.ethnicity = thing.ethnicity.toLowerCase()
    }

    switch (thing.gender) {
      case "NonBinaryThey":
        thing.gender = "non-binary"
        break
      default:
        if (thing.gender) {
          thing.gender = thing.gender.toLowerCase()
        }
    }

    switch (thing.species) {
      case "HalfElf":
        thing.species = "half-elf"
        break
      case "HalfOrc":
        thing.species = "half-orc"
        break
      default:
        if (thing.species) {
          thing.species = thing.species.toLowerCase()
        }
    }

    if (thing.subtype) {
      thing.subtype = thing.subtype.toLowerCase()
    }
  })
})

dexie.version(6).stores({
  things: "&uuid, &name, type",
  keyValue: "&key",
}).upgrade((tx) => {
  return tx.table("things").toCollection().modify((thing) => {
    if (thing.type === "Location") {
      thing.type = "Place"

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

dexie.open()

const health_check = () => !dexie.hasFailed();

const delete_thing_by_uuid = async (uuid) => {
  return dexie.things.delete(uuid)
    .then(() => true)
    .catch(() => false)
}

const get_all_the_things = async () => {
  return dexie.things.toArray()
    .catch(() => {})
}

const get_thing_by_uuid = async (uuid) => {
  return dexie.things.get({ uuid }).catch(() => {console.log('error')})
}

const get_thing_by_name = async (name) => {
  return dexie.things
    .where("name")
    .equalsIgnoreCase(name)
    .first()
    .catch(() => {})
}

const get_things_by_name_start = async (name, limit) => {
  return dexie.things
    .where("name")
    .startsWithIgnoreCase(name)
    .limit(limit)
    .toArray()
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
  get_thing_by_uuid,
  get_thing_by_name,
  get_things_by_name_start,
  get_value,
  health_check,
  save_thing,
  set_value,
}
