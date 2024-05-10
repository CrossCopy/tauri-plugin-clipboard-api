use base64::{engine::general_purpose, Engine as _};
use clipboard_rs::Clipboard;

fn main() {
    let base64_image = "iVBORw0KGgoAAAANSUhEUgAAAHgAAAB4CAYAAAA5ZDbSAAAAAXNSR0IArs4c6QAAAERlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAA6ABAAMAAAABAAEAAKACAAQAAAABAAAAeKADAAQAAAABAAAAeAAAAAAI4lXuAAAEeElEQVR4Ae2dy3IcMQwDs6n8/y8nK59cnPYWRo+ZEYI9RTQFkehVxPIhef19f37lY+vAb9vO0tiXAwFs/kUI4AA2d8C8vdzgADZ3wLy93OAANnfAvL3cYHPAf9T+Xq+Xmro0j37xVmtTclqRlKcUX89T9qzIUerPDV7h/IM0A/hBMFaUEsArXH2QZgA/CMaKUuQhiw5XHnnap8bUYaa3DtLv1erdN9uLqpcbXB0xWwewGdDaTgBXR8zWAWwGtLYzNGRVsbamwYXyamz1kFLPa2s6s9ZPOaRFsapFORQbObPq5QZXR8zWAWwGtLYTwNURs3UAmwGt7UwfsuoBV6yVYYYGF9pHeVf0sOqM3OBVzj5EN4AfAmJVGQG8ytmH6AbwQ0CsKsNiyFIGo/9hoKIvSW4wuWIUC2AjmNRKAJMrRrHpb7DyHq72j97b1WeS/hO8yA0mMkaxADaCSa0EMLliFAtgI5jUytCQ9ZRhptZBw03NITNarOaRFu2t+yjnjlhu8B2uX3hmAF9o9h1HBfAdrl94ZgBfaPYdR73eQ4Tdv1WpDjyGrR++Q7nBB0u8AgHsxfPQTQAfLPEKBLAXz0M3Q7/JOqi9AzTg0DBDeaRXYzO1qnZbK3VRDaRVY6StavXuzQ2uFMzWAWwGtLYTwNURs/X0N5jelN73g7wmrZpHNdSctiYtZW/vPkWb6hyJ5QaPuLfB3gDeANJIiQE84t4GewN4A0gjJQ4NWTRsjBRT95K+Mqj07qvnj65rHUrto2fW/bnB1RGzdQCbAa3tBHB1xGwdwGZAazvykFUHhiY0c2gY0ae9tVHKUerv3Uf+qFqUV/tR17nBqlOb5gXwpuDUsgNYdWrTvADeFJxatjxk0UCiDgO0Vylwpj5pUazWRbXTPjWv6tN6plZuMDlsFAtgI5jUSgCTK0Yx+Q2mnumtoDwlNqJFb2I9c0S/atGaaug9k7ToTCWWG6y4tHFOAG8MTyk9gBWXNs4J4I3hKaXLQ9bMh18p7KccGlxqTK2V8hStmtNqJa2fevge7933XePTn3ODP7lj8LMANoD4qYUA/uSOwc8C2ADipxbkIYtEaNigvN6YOoAoeUpOq1PNU3pStFQPFS2qKTeYXDGKBbARTGolgMkVo1gAG8GkVoaGLBLsHQbUYYPOVGK9+r39tJqUM0lf2af03HJyg1WnNs0L4E3BqWUHsOrUpnnT3+A7fKhvFr1rFKv7qHbK6dUifYrN1M8NJoeNYgFsBJNaCWByxSgWwEYwqRWLIasOJTQYUfN1H+VQrFef9lGMzqRalb25weSmUSyAjWBSKwFMrhjFAtgIJrUyfchSHn4qZCSmnNk7pFBdqlZvXXSmokX7coPJFaNYABvBpFYCmFwxigWwEUxqZWjIomGDDrk6ptal5PUON63nqj9Tq+krernBzSnjTwAbw22tBXAAmztg3p7lf/FuzuxUe/kr+pRd+yUH8H7MTlUcwKfs2i85gPdjdqriAD5l137JAbwfs1MVB/Apu/ZLDuD9mJ2q+B9DYv0QlD6gxgAAAABJRU5ErkJggg==";
    let ctx = clipboard_rs::ClipboardContext::new().unwrap();
    // ctx.set_image(image)
    let decoded = general_purpose::STANDARD
        .decode(base64_image)
        .map_err(|err| err.to_string())
        .unwrap();
    
    // self.write_image_binary(decoded)
    //     .map_err(|err| err.to_string())?;
}