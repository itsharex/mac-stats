#!/usr/bin/env python3
"""Query Redmine API for time entries."""
import json
import os
import sys
import urllib.request

def read_config():
    config = {}
    for path in [
        os.path.join(os.path.dirname(__file__), '..', 'src-tauri', '.config.env'),
        os.path.expanduser('~/.mac-stats/.config.env'),
    ]:
        if os.path.exists(path):
            with open(path) as f:
                for line in f:
                    line = line.strip()
                    if '=' in line and not line.startswith('#'):
                        key, val = line.split('=', 1)
                        config[key.strip()] = val.strip().strip("'\"")
    return config

def redmine_get(config, path):
    url = f"{config['REDMINE_URL'].rstrip('/')}/{path.lstrip('/')}"
    req = urllib.request.Request(url, headers={
        'X-Redmine-API-Key': config['REDMINE_API_KEY'],
        'User-Agent': 'mac-stats-query',
    })
    with urllib.request.urlopen(req, timeout=20) as resp:
        return json.loads(resp.read())

def main():
    config = read_config()
    if 'REDMINE_URL' not in config or 'REDMINE_API_KEY' not in config:
        print("Redmine not configured")
        sys.exit(1)

    action = sys.argv[1] if len(sys.argv) > 1 else 'users'

    if action == 'users':
        data = redmine_get(config, '/users.json?limit=100')
        for u in data.get('users', []):
            name = f"{u.get('firstname', '')} {u.get('lastname', '')}"
            print(f"  id={u['id']}: {name} (login={u.get('login', '')})")

    elif action == 'time':
        from_date = sys.argv[2] if len(sys.argv) > 2 else '2026-03-09'
        to_date = sys.argv[3] if len(sys.argv) > 3 else '2026-03-15'
        user_id = sys.argv[4] if len(sys.argv) > 4 else ''
        path = f'/time_entries.json?from={from_date}&to={to_date}&limit=100'
        if user_id:
            path += f'&user_id={user_id}'
        data = redmine_get(config, path)
        entries = data.get('time_entries', [])
        total = data.get('total_count', len(entries))
        print(f"Total entries: {total}, fetched: {len(entries)}")
        print(f"Date range: {from_date} to {to_date}")
        print()

        # Group by ticket
        tickets = {}
        total_hours = 0
        for e in entries:
            total_hours += e.get('hours', 0)
            issue = e.get('issue', {})
            issue_id = issue.get('id', 0)
            user = e.get('user', {}).get('name', 'unknown')
            project = e.get('project', {}).get('name', 'unknown')
            activity = e.get('activity', {}).get('name', 'unknown')
            key = issue_id if issue_id else f"no-issue-{project}-{activity}"
            if key not in tickets:
                tickets[key] = {
                    'issue_id': issue_id,
                    'subject': issue.get('subject', ''),
                    'project': project,
                    'hours': 0,
                    'entries': [],
                    'users': set(),
                    'activities': set(),
                }
            tickets[key]['hours'] += e.get('hours', 0)
            tickets[key]['users'].add(user)
            tickets[key]['activities'].add(activity)
            tickets[key]['entries'].append(e)

        print(f"Total hours: {total_hours:.2f}")
        print()
        print("=== Tickets worked on ===")
        for key, t in sorted(tickets.items(), key=lambda x: -x[1]['hours']):
            if t['issue_id']:
                print(f"  #{t['issue_id']} {t['subject']} — {t['hours']:.2f}h ({t['project']})")
            else:
                print(f"  (no ticket) {t['project']} — {t['hours']:.2f}h")
            for act in t['activities']:
                print(f"    Activity: {act}")
            for u in t['users']:
                print(f"    User: {u}")

        print()
        print("=== Day-by-day details ===")
        by_date = {}
        for e in entries:
            day = e.get('spent_on', 'unknown')
            by_date.setdefault(day, []).append(e)
        for day in sorted(by_date.keys()):
            day_hours = sum(e.get('hours', 0) for e in by_date[day])
            print(f"\n  {day} ({day_hours:.2f}h):")
            for e in by_date[day]:
                issue = e.get('issue', {})
                issue_str = f"#{issue['id']}" if issue.get('id') else "(no ticket)"
                comments = e.get('comments', '') or ''
                activity = e.get('activity', {}).get('name', '')
                print(f"    {issue_str} — {e.get('hours', 0):.2f}h — {activity}{' — ' + comments if comments else ''}")

if __name__ == '__main__':
    main()
