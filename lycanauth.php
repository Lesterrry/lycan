<?php
error_reporting(E_ALL);
ini_set('display_errors', 1);
$lykey = shell_exec('sudo lycan -p -r');
if($_GET["key"] == trim(preg_replace('/\s\s+/', '', $lykey))){
        echo('1');
}
else{
        echo('0');
}
