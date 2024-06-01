#include "SimpleSleep.h"

// Sleep until next check
void sleepUntil(const String& check_time) {
  // we expect the time to be in the format "HH:MM"
  int targetHour = check_time.substring(0, 2).toInt();
  int targetMinute = check_time.substring(3, 5).toInt();

  time_t now = time(nullptr);
  struct tm *now_tm = localtime(&now);

  // Setup target time: from config
  // add one day
  struct tm next_tm = *now_tm;

  next_tm.tm_hour = targetHour;
  next_tm.tm_min = targetMinute;
  next_tm.tm_sec = 0;
  next_tm.tm_mday += 1;

  time_t next_time = mktime(&next_tm);
  double secondsUntilTarget = difftime(next_time, now);

  // Format next time as HH:MM
  char formattedTime[6];
  strftime(formattedTime, sizeof(formattedTime), "%H:%M", &next_tm);
  info("Next check time: ", formattedTime, " - see you in ", secondsUntilTarget / 60, " minutes");

  // Set deep sleep in microseconds
  debug("Going to sleep now... (", secondsUntilTarget / 60, "minutes)");
  delay(1000);
  ESP.deepSleep((uint64_t)(secondsUntilTarget * 1000000));
}
