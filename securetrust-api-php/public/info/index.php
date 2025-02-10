<?php

$found = false;
if (isset($_GET['p'])) {
    $passwords = explode("\n", file_get_contents("../../data/passwords.lst"));
    $password = $_GET['p'];
    foreach ($passwords as $pass) {
        if ($pass == $password) {
            $found = true;
        }
    }

    $info = "This is a SecureTrust node.";
    $contact = "";
    $infos = array_map(function ($i) {
        return explode("=", $i);
    }, explode("\n", file_get_contents("../../data/info.cfg")));
    foreach ($infos as $i) {
        if ($i[0] == "contact") {
            $contact = $i[1];
        } elseif ($i[0] == "info") {
            $info = $i[1];
        }
    }
}

echo "{ status = \"ok\", result = { info = \"" . $info . "\", contact = \"" . $contact . "\", ";
if (!$found) {
    echo "features = { \"list\" }";
} else {
    echo "features = { \"list\", \"add\", \"remove\" }";
}
echo " } }";
