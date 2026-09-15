-- 1. v_taryf v_mtghyrat v_wastkhdam v_alqwaad v_alshrtyt v_walmntqyt
local v_alntyjt = true
local v_ns_tjryby = "لغة luu"

if v_alntyjt == true and not false then
    print("1. نجاح اختبار القواعد والرموز المنطقية")
end

-- 2. v_taryf v_dalt v_wakhtbar v_almaamlat v_walmsfwfat
function v_hsab_almjmwa(v_a, v_b)
    return v_a + v_b
end

local v_mjmwa = v_hsab_almjmwa(10, 20)
if v_mjmwa == 30 then
    print("2. نجاح استدعاء الدوال والفاصلة العربية")
end

-- 3. v_akhtbar v_aljdawl (Tables) v_wdwal v_altkrar v_walklmt v_almftahyt (in)
local v_msfwft = {"الأول", "الثاني", "الثالث"}

for v_mftah, v_qymt in pairs(v_msfwft) do
    if v_mftah == 2 then
        print("3. نجاح حلقة التكرار (في) والأزواج:")
        print(v_qymt)
    end
end

-- 4. v_akhtbar v_hlqat v_altkrar (while) and (for)
local v_adad = 1
while v_adad <= 2 do
    v_adad = v_adad + 1
end

for v_s = 1, 1 do
    print("4. نجاح تنفيذ حلقة (لطالما) وحلقة (لـ)")
end

-- 5. v_akhtbar v_aljdawl v_alfaqt (Metatables) v_walrmwz v_aljryt (rawget / rawset)
local v_jdwl_asly = {}
local v_jdwl_qwaad = {}

setmetatable(v_jdwl_asly, v_jdwl_qwaad)

if not (getmetatable(v_jdwl_asly) == nil) then
    print("5. نجاح تعيين وفحص الجدول الفائق (جدول_فائقًا)")
end

rawset(v_jdwl_asly, "مفتاح_خام", "قيمة_خام")
local v_qymt_mstrjat = rawget(v_jdwl_asly, "مفتاح_خام")

if v_qymt_mstrjat == "قيمة_خام" then
    print("6. نجاح معالجة (أفلت) و(آتنِ) للوصول المباشر")
end

-- 6. v_akhtbar v_mktbt v_alnzam (v_falmshghl) v_walwqt
local v_alwqt_alhaly = os.time()
if not (v_alwqt_alhaly == nil) then
    print("7. نجاح الاستعلام عن النظام عبر (الوقت.فالمشغل):")
    print(v_alwqt_alhaly)
end
