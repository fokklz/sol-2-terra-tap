#ifndef SIMPLE_DEBUG_H
#define SIMPLE_DEBUG_H

#include <Arduino.h>

// Allow the debug flag to be defined externally
extern bool DEBUG_ENABLED;

// Empty function definitions for base cases
inline void print() {
  // Base case for recursion
}

// Empty function definitions to print a newline
inline void println() {
  Serial.println();
}

// Recursive function to print multiple arguments
template<typename T, typename... Args>
void print(T first, Args... args) {
  Serial.print(first);
  Serial.print(' ');
  print(args...);
}

// Recursive function to print multiple arguments with a newline
template<typename T, typename... Args>
void println(T first, Args... args) {
  Serial.print(first);
  Serial.print(' ');
  print(args...);
  Serial.println();
}

// Debug function that only prints if debugging is enabled
template<typename... Args>
void debug(Args... args) {
  if (DEBUG_ENABLED) {
    println(args...);
  }
}

// Info function that always prints
template<typename... Args>
void info(Args... args) {
  println(args...);
}

#endif // SIMPLE_DEBUG_H
