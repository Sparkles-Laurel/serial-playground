#define THE_LED 8
#define SYMBOL_RATE_OF_CLIENT 9600

void setup() {
  pinMode(THE_LED, OUTPUT);
  if(Serial) {
    Serial.begin(SYMBOL_RATE_OF_CLIENT);
  }
}

void loop() {
  if(Serial && Serial.available()) {
    auto input = Serial.readStringUntil('\n');
    Serial.print("USER: ");
    Serial.println(input);
    if(input == "i gay") {
      Serial.println("wish i had an RGB led for you :(");
      return;
    }
    input.toUpperCase();
    if(input == "HIGH") {
      digitalWrite(THE_LED, HIGH);
      Serial.println("LED #" + THE_LED + " is set to HIGH :3");
    } else if(input == "LOW") {
      digitalWrite(THE_LED, LOW);
      Serial.println("LED #" + THE_LED + " is set to LOW :3");
    } else if(input == "TOGGLE") {
      digitalWrite(THE_LED, !digitalRead(THE_LED));
      Serial.print("Toggling LED #" + THE_LED ", the state of the led is ");
      Serial.print(digitalRead(THE_LED) == LOW ? "LOW" : "HIGH");
      Serial.println(" :3");
    } else {
      Serial.println("Invalid input, please try again :3");
    }
  }
}