import Dexie from "dexie"
import * as download from "downloadjs"
import * as wasm from "initiative-web"

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

export function health_check() {
  return !dexie.hasFailed()
}

export async function delete_thing_by_uuid(uuid) {
  return dexie.things.delete(uuid)
}

export async function get_all_the_things() {
  return dexie.things.toArray()
}

export async function get_thing_by_uuid(uuid) {
  return dexie.things.get({ uuid })
}

export async function get_thing_by_name(name) {
  return dexie.things
    .where("name")
    .equalsIgnoreCase(name)
    .first()
}

export async function get_things_by_name_start(name, limit) {
  return dexie.things
    .where("name")
    .startsWithIgnoreCase(name)
    .limit(limit)
    .toArray()
}

export async function save_thing(thing) {
  return dexie.things.put(thing)
}

export async function set_value(key, value) {
  return dexie.keyValue.put({key, value})
}

export async function get_value(key) {
  return dexie.keyValue.get(key).then((v) => v?.value)
}

export async function delete_value(key) {
  return dexie.keyValue.delete(key)
}

export async function export_database(data) {
  download(JSON.stringify(data), "initiative_export.json", "application/json")
}

export async function import_database(successCallback, failureCallback) {
  const inputElement = document.createElement("input")
  inputElement.accept = "application/json"
  inputElement.style = "display: none"
  inputElement.type = "file"

  inputElement.addEventListener("change", async (event) => {
    if (event.target.files.length !== 1) {
      failureCallback("Please select a file to import.")
      return
    }

    const file = event.target.files[0]

    if (!/\.json$/.test(file.name)) {
      failureCallback("The file you selected does not appear to be JSON.")
      return
    }

    const reader = new FileReader()
    reader.addEventListener("loadstart", (event) => console.log(event))
    reader.addEventListener("error", (event) => console.error(event))
    reader.addEventListener("load", (event) => {
      try {
        const data = JSON.parse(event.target.result)
        successCallback(data)
      } catch (e) {
        failureCallback("The file you selected does not appear to be JSON.")
      }
    })
    reader.readAsText(file)
  })

  document.body.insertAdjacentElement("beforeend", inputElement)
  inputElement.click()
  inputElement.remove()
}
