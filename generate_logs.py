import time
import random
import json
from datetime import datetime

LOG_FILE = "test.log"
LEVELS = ["INFO", "WARN", "ERROR", "DEBUG"]

def generate_plain_log():
    level = random.choice(LEVELS)
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    messages = [
        "User logged in",
        "Database connection established",
        "Failed to fetch resource",
        "Disk space low",
        "Optimizing cache",
        "Request processed in 45ms"
    ]
    return f"[{timestamp}] [{level}] {random.choice(messages)}"

def generate_json_log():
    level = random.choice(LEVELS)
    messages = [
        "API request received",
        "Token expired",
        "Job completed",
        "Service unavailable"
    ]
    log_data = {
        "timestamp": datetime.now().isoformat(),
        "level": level,
        "message": random.choice(messages),
        "module": "auth_service"
    }
    return json.dumps(log_data)

print(f"Generating logs to {LOG_FILE}... Press Ctrl+C to stop.")
with open(LOG_FILE, "a") as f:
    try:
        while True:
            log_line = generate_json_log() if random.random() > 0.5 else generate_plain_log()
            f.write(log_line + "\n")
            f.flush()
            time.sleep(random.uniform(0.1, 0.5))
    except KeyboardInterrupt:
        print("\nStopped log generation.")
