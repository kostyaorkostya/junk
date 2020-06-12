#!/usr/bin/env python

__author__ = 'yazevnul'

import argparse
import iso8601

import xml.etree.ElementTree as et


def _parse_options():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description='`svn log --xml` output parser',
        epilog=('Parse `svn log --xml` output into machine TSV format for further analysis.\n'
                + 'To retrieve log for a fixed period of time read more about {-r|--revision}'
                + ' option in `svn log --help`.\n\n'
                + ' Originally posted at'
                + ' https://github.com/yazevnul/junk/blob/master/parse_svn_log.py')
    )
    """
    svn log --revision {"2015-09-02 00:00:01 +0300"}:{"2015-09-05 00:00:01 +0300"} --xml
    """
    parser.add_argument(
        '-i', '--input',
        dest='input_file',
        metavar='FILE',
        default='-',
        type=argparse.FileType('r'),
        help='`svn log --xml` output'
    )
    parser.add_argument(
        '-o', '--output',
        dest='output_file',
        metavar='FILE',
        default='-',
        type=argparse.FileType('w'),
        help='revision <TAB> author <TAB> ISO date'
    )
    return parser.parse_args()


def _main(args):
    tree = et.fromstring(args.input_file.read())
    for log_entry in tree:
        revision = log_entry.attrib['revision']
        for field in log_entry:
            if 'author' == field.tag:
                author = field.text
            elif 'date' == field.tag:
                date = iso8601.parse_date(field.text).isoformat(' ')

        args.output_file.write('{}\t{}\t{}\n'.format(revision, author, date))

if '__main__' == __name__:
    args = _parse_options()
    _main(args)
