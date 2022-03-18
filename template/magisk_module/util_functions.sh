require_new_android() {
  ui_print "*********************************************************"
  ui_print "! Unsupported Android version ${1} (below Oreo MR1)"
  abort    "*********************************************************"
}

check_android_version() {
  if [ "$API" -ge 23 ]; then
    ui_print "- Android SDK version: $API"
  else
    require_new_android "$API"
  fi
}
