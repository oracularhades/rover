import { Rover } from "@oracularhades/rover";
const creds = {
    device_id: "DEVICE_ID",
    private_key: "PUBLIC_KEY",
    host: "http://127.0.0.1:8012/api"
}

// ---- Users ----
// List users
await Rover(creds).user.list();

const user = {
    first_name: "John",
    last_name: "Doe",
    email: "john.doe24@example.com",
    permission: 0
};

console.log("user", user);

// Create user
const user_create = await Rover(creds).user.create(user);
console.log("user_create", user_create);

try {
    await Rover(creds).user.create(user);
    throw "bad";
} catch (error) {
    if (error == "bad") {
        throw `We were able to create a user with an in-use email address, that isn't good! - Sent: ${JSON.stringify(user)}, Received: ${JSON.stringify(user_create)}`;
    } else {
        // This is good news! - We can't create an account with an email that is already in-use.
    }
}

try {
    await await Rover(creds).user.update({ id: user_create.user_id, email: user.email, first_name: "placeholder", last_name: "placeholder", permission: 0 });;
    throw "bad";
} catch (error) {
    console.log("ERRRR", error);
    if (error == "bad") {
        throw `We were able to update a user with an in-use email address, that isn't good! - Sent: ${JSON.stringify(user)}, Received: ${JSON.stringify(user_create)}`;
    } else {
        // This is good news! - We can't update an account with an email that is already in-use.
    }
}

const user_get = await Rover(creds).user.get(user_create.user_id);
console.log("user_get", user_get);

const update_user = Rover().general().sortedObject({
    id: user_create.user_id,
    first_name: "John1",
    last_name: "Doe1",
    email: "john.doe.23@example.com",
    permission: 0
});

console.log("update_user11", update_user);

await Rover(creds).user.update(update_user);

// Make sure our updated worked.
const user_test_updated = await Rover(creds).user.get(user_create.user_id);
console.log(user_test_updated);

// Make sure the user we requested was found.
if (user_test_updated.data.length != 1) {
    throw `/user/get should have returned 1 user, instead it returned ${user_test_updated.data.length}`;
}

// Get our created user from the returned array.
const output_user = Rover().general().sortedObject(user_test_updated.data[0])
if (JSON.stringify(output_user) != JSON.stringify(update_user)) {
    throw `Attempted to update user data, but the returned user object differs from the update. This may be because there are additional user fields the test script is not expecting. \nExpecting: ${JSON.stringify(update_user)} \n... \ngot: ${JSON.stringify(output_user)}`;
}

// ---- Network ----
await Rover(creds).network.list();

// ---- Processes ----
await Rover(creds).process.list();