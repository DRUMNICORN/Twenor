// app/api/audio.ts

export default function handler(req, res) {
    fetch("http://localhost:8000/api/audio", {
        method: "POST",
        headers: {
            'Content-Type': 'application/json',
            // Include other headers as needed
        },
        body: JSON.stringify({
            // Your request body here
        })
    })
        .then(response => response.json())
        .then(data => res.status(200).json(data))
        .catch((error) => {
        });
}
