<?php
if (!isset($_GET['target'])) {
    echo "{ status = \"error\", error = \"No ban target specified\" }";
    die();
}
if (!isset($_GET['p'])) {
    echo "{ status = \"error\", error = \"No password specified\" }";
    die();
}

$passwords = explode("\n", file_get_contents("../../data/passwords.lst"));
$password = $_GET['p'];
$found = false;
foreach ($passwords as $pass) {
    if ($pass == $password) {
        $found = true;
    }
}
if (!$found) {
    echo "{ status = \"error\", error = \"Invalid password\" }";
    die();
}

$target = $_GET['target'];
$reason = $_GET['reason'] ?? "";
$time = $_GET['time'] ?? "";

$bans = explode("\n", file_get_contents("../../data/bans.lst"));
$newBans = "";
foreach ($bans as $ban) {
    if (trim($ban) != "" && !str_contains($ban, $target . "|")) {
        $newBans .= $ban . "\n";
    }
}
$newBans .= $target . "|" . $reason . "|" . $time . "\n";

file_put_contents("../../data/bans.lst", $newBans);

echo "{ status = \"ok\", result = \"" . $target . "\" }";
