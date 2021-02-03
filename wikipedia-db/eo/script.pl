#! /usr/bin/perl
 
use strict;
 
open(READ,$ARGV[0]);
open(WRITE,">output_".$ARGV[0]);
my $count=0;
my $is_first_line = "yes";
while(<READ>){
 
my $line=$_;
chomp $line;
if ($is_first_line eq "yes"){
$is_first_line = "no";
print WRITE "BEGIN TRANSACTION;\n";
}
 
print WRITE $line."\n";
$count++;
 
if ($count % 10000 == 0){
print WRITE "END TRANSACTION;\n";
print WRITE "BEGIN TRANSACTION;\n";
}
}
close READ;
print WRITE "END TRANSACTION\n";
close WRITE;