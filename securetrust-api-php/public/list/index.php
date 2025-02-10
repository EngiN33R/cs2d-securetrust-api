<?php
$bans = explode("\n", file_get_contents("../../data/bans.lst"));

echo "{ status = \"ok\", result = { ";

echo join(
    ", ",
    array_map(
        function ($ban) {
            $ban = explode("|", $ban);
            return "{ target = \"" . $ban[0] . "\", reason = " . ($ban[1] == "" ? "nil" : "\"" . $ban[1] . "\"") . ", time = " . ($ban[2] == "" ? -1 : $ban[2]) . " }";
        },
        array_filter(
            $bans,
            function ($ban) {
                return $ban != "";
            }
        )
    )
);

echo " } } ";
