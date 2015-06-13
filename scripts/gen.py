import json

data = {}
with open("openvr/headers/openvr_api.json") as f:
	data = json.loads(f.read())

type_mapping = {
	'uint64_t': 'u64',
	'uint32_t': 'u32',
	'uint16_t': 'u16',
	'uint8_t': 'u8',
	'int64_t': 'i64',
	'int32_t': 'i32',
	'int16_t': 'i16',
	'int8_t': 'i8',
	'double': 'f64',
	'float': 'f32',
	'_Bool': 'bool',
	'unsigned short': 'u16',
	'const char': 'u8',
	'void': '()',

	# I'm lazy
	'unsigned char *': '*const u8',
	'char *': '*const u8',
	'const uint16_t *': '*const u16',
	'const uint8_t *': '*const u8',
	'const struct vr::HmdVector2_t *': '*const HmdVector2_t',
	'const struct vr::RenderModel_Vertex_t *': '*const RenderModel_Vertex_t',

	'float [3][4]': '[[f32; 4]; 3]',
	'float [4][4]': '[[f32; 4]; 4]',
	'float [4]': '[f32; 4]',
	'float [3]': '[f32; 3]',
	'float [2]': '[f32; 2]',
	'double [3]': '[f64; 3]',

	'union VREvent_Data_t': '[u8; 16]'
}


def parse_type(s):
	if s.startswith("struct"):
		return parse_type(s.split()[1])
	elif s.startswith("vr::"):
		return parse_type(s[4:])
	elif s.startswith('enum'):
		return parse_type(s.split()[1])
	elif s.startswith("const "):
		return parse_type(s[6:])
	elif s in type_mapping:
		return type_mapping[s]
	elif s[-2:] == ' *':
		return "*mut "  + parse_type(s[:-2])
	return s

def parse_class(s):
	if s.startswith("vr::"):
		return 'VR_' + s[4:]
	return s


def shorten_enum(parent, name):
	split = name.split('_')
	if len(split) == 2:
		return split[-1]
	elif len(split) > 2:
		return '_'.join(split[1:])
	return name



print """
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[link(name = "openvr_api")]
extern {}
"""

for d in data['typedefs']:
	if parse_type(d['typedef']) == parse_type(d['type']):
		continue

	print "pub type %s = %s;" % (parse_type(d['typedef']), parse_type(d['type']))

for d in data['enums']:
	found = set()
	print "#[repr(C)]\npub enum %s {" % parse_type(d['enumname'])
	for v in d['values']:
		if v['value'] in found:
			continue
		found.add(v['value'])
		print "\t%s = %s," % (shorten_enum(d['enumname'], v['name']), v['value'])
	print "}\n"

for s in data['structs']:
	if s['struct'] == "vr::(anonymous)":
		continue
	print "#[repr(C)]\npub struct %s {" % parse_type(s['struct'])
	for f in s['fields']:
		print "\t%s: %s," % (f['fieldname'], parse_type(f['fieldtype']))
	print "}"

print "extern \"C\" {"

for m in data['methods']:
	print '\tpub fn ' + parse_class(m['classname']) + '_' + m['methodname'] + "(usize,",
	s = []
	for p in m.get('params', []):
		if p['paramname'] == 'type':
			p['paramname'] = '_type'
		s +=  ["%s: %s" % (p['paramname'], parse_type(p['paramtype']))]
	print "%s)" % (', '.join(s)),
	if 'returntype' in m and m['returntype'] == 'void':
		print ";"
	elif 'returntype' in m:
		print "-> %s;" % parse_type(m['returntype'])
	else:
		print ";"

print "}"


