// there's a better way to do this, but this is fastest to write
const BIN: &'static [&'static [u8]] =
    &[include_bytes!("common/data/pairs/doubled-bytes/case0-0.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case100-100.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case10-10.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case101-101.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case102-102.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case103-103.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case104-104.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case105-105.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case106-106.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case107-107.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case108-108.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case109-109.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case110-110.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case111-111.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case11-11.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case112-112.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case113-113.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case114-114.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case115-115.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case116-116.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case117-117.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case118-118.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case119-119.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case1-1.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case120-120.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case121-121.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case12-12.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case122-122.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case123-123.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case124-124.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case125-125.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case126-126.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case127-127.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case128-128.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case129-129.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case130-130.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case131-131.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case13-13.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case132-132.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case133-133.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case134-134.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case135-135.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case136-136.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case137-137.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case138-138.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case139-139.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case140-140.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case141-141.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case14-14.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case142-142.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case143-143.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case144-144.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case145-145.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case146-146.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case147-147.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case148-148.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case149-149.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case150-150.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case151-151.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case15-15.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case152-152.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case153-153.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case154-154.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case155-155.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case156-156.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case157-157.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case158-158.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case159-159.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case160-160.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case161-161.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case16-16.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case162-162.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case163-163.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case164-164.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case165-165.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case166-166.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case167-167.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case168-168.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case169-169.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case170-170.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case171-171.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case17-17.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case172-172.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case173-173.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case174-174.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case175-175.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case176-176.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case177-177.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case178-178.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case179-179.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case180-180.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case181-181.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case18-18.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case182-182.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case183-183.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case184-184.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case185-185.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case186-186.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case187-187.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case188-188.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case189-189.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case190-190.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case191-191.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case19-19.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case192-192.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case193-193.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case194-194.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case195-195.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case196-196.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case197-197.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case198-198.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case199-199.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case200-200.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case201-201.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case20-20.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case202-202.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case203-203.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case204-204.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case205-205.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case206-206.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case207-207.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case208-208.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case209-209.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case210-210.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case211-211.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case21-21.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case212-212.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case213-213.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case214-214.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case215-215.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case216-216.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case217-217.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case218-218.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case219-219.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case220-220.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case221-221.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case222-222.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case22-22.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case223-223.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case224-224.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case225-225.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case226-226.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case227-227.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case228-228.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case229-229.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case2-2.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case230-230.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case231-231.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case232-232.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case23-23.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case233-233.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case234-234.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case235-235.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case236-236.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case237-237.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case238-238.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case239-239.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case240-240.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case241-241.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case242-242.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case24-24.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case243-243.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case244-244.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case245-245.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case246-246.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case247-247.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case248-248.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case249-249.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case250-250.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case251-251.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case252-252.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case25-25.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case253-253.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case254-254.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case255-255.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case26-26.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case27-27.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case28-28.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case29-29.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case30-30.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case31-31.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case32-32.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case33-33.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case3-3.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case34-34.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case35-35.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case36-36.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case37-37.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case38-38.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case39-39.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case40-40.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case41-41.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case42-42.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case43-43.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case44-44.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case4-4.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case45-45.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case46-46.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case47-47.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case48-48.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case49-49.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case50-50.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case51-51.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case52-52.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case53-53.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case54-54.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case55-55.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case5-5.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case56-56.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case57-57.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case58-58.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case59-59.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case60-60.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case61-61.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case62-62.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case63-63.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case64-64.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case65-65.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case66-66.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case6-6.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case67-67.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case68-68.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case69-69.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case70-70.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case71-71.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case72-72.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case73-73.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case74-74.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case75-75.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case76-76.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case77-77.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case7-7.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case78-78.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case79-79.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case80-80.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case81-81.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case82-82.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case83-83.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case84-84.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case85-85.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case86-86.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case87-87.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case88-88.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case8-8.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case89-89.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case90-90.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case91-91.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case92-92.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case93-93.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case94-94.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case95-95.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case96-96.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case97-97.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case98-98.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case99-99.bin"),
      include_bytes!("common/data/pairs/doubled-bytes/case9-9.bin")];
const TXT: &'static [&'static str] =
    &[include_str!("common/data/pairs/doubled-bytes/case0-0.txt"),
      include_str!("common/data/pairs/doubled-bytes/case100-100.txt"),
      include_str!("common/data/pairs/doubled-bytes/case10-10.txt"),
      include_str!("common/data/pairs/doubled-bytes/case101-101.txt"),
      include_str!("common/data/pairs/doubled-bytes/case102-102.txt"),
      include_str!("common/data/pairs/doubled-bytes/case103-103.txt"),
      include_str!("common/data/pairs/doubled-bytes/case104-104.txt"),
      include_str!("common/data/pairs/doubled-bytes/case105-105.txt"),
      include_str!("common/data/pairs/doubled-bytes/case106-106.txt"),
      include_str!("common/data/pairs/doubled-bytes/case107-107.txt"),
      include_str!("common/data/pairs/doubled-bytes/case108-108.txt"),
      include_str!("common/data/pairs/doubled-bytes/case109-109.txt"),
      include_str!("common/data/pairs/doubled-bytes/case110-110.txt"),
      include_str!("common/data/pairs/doubled-bytes/case111-111.txt"),
      include_str!("common/data/pairs/doubled-bytes/case11-11.txt"),
      include_str!("common/data/pairs/doubled-bytes/case112-112.txt"),
      include_str!("common/data/pairs/doubled-bytes/case113-113.txt"),
      include_str!("common/data/pairs/doubled-bytes/case114-114.txt"),
      include_str!("common/data/pairs/doubled-bytes/case115-115.txt"),
      include_str!("common/data/pairs/doubled-bytes/case116-116.txt"),
      include_str!("common/data/pairs/doubled-bytes/case117-117.txt"),
      include_str!("common/data/pairs/doubled-bytes/case118-118.txt"),
      include_str!("common/data/pairs/doubled-bytes/case119-119.txt"),
      include_str!("common/data/pairs/doubled-bytes/case1-1.txt"),
      include_str!("common/data/pairs/doubled-bytes/case120-120.txt"),
      include_str!("common/data/pairs/doubled-bytes/case121-121.txt"),
      include_str!("common/data/pairs/doubled-bytes/case12-12.txt"),
      include_str!("common/data/pairs/doubled-bytes/case122-122.txt"),
      include_str!("common/data/pairs/doubled-bytes/case123-123.txt"),
      include_str!("common/data/pairs/doubled-bytes/case124-124.txt"),
      include_str!("common/data/pairs/doubled-bytes/case125-125.txt"),
      include_str!("common/data/pairs/doubled-bytes/case126-126.txt"),
      include_str!("common/data/pairs/doubled-bytes/case127-127.txt"),
      include_str!("common/data/pairs/doubled-bytes/case128-128.txt"),
      include_str!("common/data/pairs/doubled-bytes/case129-129.txt"),
      include_str!("common/data/pairs/doubled-bytes/case130-130.txt"),
      include_str!("common/data/pairs/doubled-bytes/case131-131.txt"),
      include_str!("common/data/pairs/doubled-bytes/case13-13.txt"),
      include_str!("common/data/pairs/doubled-bytes/case132-132.txt"),
      include_str!("common/data/pairs/doubled-bytes/case133-133.txt"),
      include_str!("common/data/pairs/doubled-bytes/case134-134.txt"),
      include_str!("common/data/pairs/doubled-bytes/case135-135.txt"),
      include_str!("common/data/pairs/doubled-bytes/case136-136.txt"),
      include_str!("common/data/pairs/doubled-bytes/case137-137.txt"),
      include_str!("common/data/pairs/doubled-bytes/case138-138.txt"),
      include_str!("common/data/pairs/doubled-bytes/case139-139.txt"),
      include_str!("common/data/pairs/doubled-bytes/case140-140.txt"),
      include_str!("common/data/pairs/doubled-bytes/case141-141.txt"),
      include_str!("common/data/pairs/doubled-bytes/case14-14.txt"),
      include_str!("common/data/pairs/doubled-bytes/case142-142.txt"),
      include_str!("common/data/pairs/doubled-bytes/case143-143.txt"),
      include_str!("common/data/pairs/doubled-bytes/case144-144.txt"),
      include_str!("common/data/pairs/doubled-bytes/case145-145.txt"),
      include_str!("common/data/pairs/doubled-bytes/case146-146.txt"),
      include_str!("common/data/pairs/doubled-bytes/case147-147.txt"),
      include_str!("common/data/pairs/doubled-bytes/case148-148.txt"),
      include_str!("common/data/pairs/doubled-bytes/case149-149.txt"),
      include_str!("common/data/pairs/doubled-bytes/case150-150.txt"),
      include_str!("common/data/pairs/doubled-bytes/case151-151.txt"),
      include_str!("common/data/pairs/doubled-bytes/case15-15.txt"),
      include_str!("common/data/pairs/doubled-bytes/case152-152.txt"),
      include_str!("common/data/pairs/doubled-bytes/case153-153.txt"),
      include_str!("common/data/pairs/doubled-bytes/case154-154.txt"),
      include_str!("common/data/pairs/doubled-bytes/case155-155.txt"),
      include_str!("common/data/pairs/doubled-bytes/case156-156.txt"),
      include_str!("common/data/pairs/doubled-bytes/case157-157.txt"),
      include_str!("common/data/pairs/doubled-bytes/case158-158.txt"),
      include_str!("common/data/pairs/doubled-bytes/case159-159.txt"),
      include_str!("common/data/pairs/doubled-bytes/case160-160.txt"),
      include_str!("common/data/pairs/doubled-bytes/case161-161.txt"),
      include_str!("common/data/pairs/doubled-bytes/case16-16.txt"),
      include_str!("common/data/pairs/doubled-bytes/case162-162.txt"),
      include_str!("common/data/pairs/doubled-bytes/case163-163.txt"),
      include_str!("common/data/pairs/doubled-bytes/case164-164.txt"),
      include_str!("common/data/pairs/doubled-bytes/case165-165.txt"),
      include_str!("common/data/pairs/doubled-bytes/case166-166.txt"),
      include_str!("common/data/pairs/doubled-bytes/case167-167.txt"),
      include_str!("common/data/pairs/doubled-bytes/case168-168.txt"),
      include_str!("common/data/pairs/doubled-bytes/case169-169.txt"),
      include_str!("common/data/pairs/doubled-bytes/case170-170.txt"),
      include_str!("common/data/pairs/doubled-bytes/case171-171.txt"),
      include_str!("common/data/pairs/doubled-bytes/case17-17.txt"),
      include_str!("common/data/pairs/doubled-bytes/case172-172.txt"),
      include_str!("common/data/pairs/doubled-bytes/case173-173.txt"),
      include_str!("common/data/pairs/doubled-bytes/case174-174.txt"),
      include_str!("common/data/pairs/doubled-bytes/case175-175.txt"),
      include_str!("common/data/pairs/doubled-bytes/case176-176.txt"),
      include_str!("common/data/pairs/doubled-bytes/case177-177.txt"),
      include_str!("common/data/pairs/doubled-bytes/case178-178.txt"),
      include_str!("common/data/pairs/doubled-bytes/case179-179.txt"),
      include_str!("common/data/pairs/doubled-bytes/case180-180.txt"),
      include_str!("common/data/pairs/doubled-bytes/case181-181.txt"),
      include_str!("common/data/pairs/doubled-bytes/case18-18.txt"),
      include_str!("common/data/pairs/doubled-bytes/case182-182.txt"),
      include_str!("common/data/pairs/doubled-bytes/case183-183.txt"),
      include_str!("common/data/pairs/doubled-bytes/case184-184.txt"),
      include_str!("common/data/pairs/doubled-bytes/case185-185.txt"),
      include_str!("common/data/pairs/doubled-bytes/case186-186.txt"),
      include_str!("common/data/pairs/doubled-bytes/case187-187.txt"),
      include_str!("common/data/pairs/doubled-bytes/case188-188.txt"),
      include_str!("common/data/pairs/doubled-bytes/case189-189.txt"),
      include_str!("common/data/pairs/doubled-bytes/case190-190.txt"),
      include_str!("common/data/pairs/doubled-bytes/case191-191.txt"),
      include_str!("common/data/pairs/doubled-bytes/case19-19.txt"),
      include_str!("common/data/pairs/doubled-bytes/case192-192.txt"),
      include_str!("common/data/pairs/doubled-bytes/case193-193.txt"),
      include_str!("common/data/pairs/doubled-bytes/case194-194.txt"),
      include_str!("common/data/pairs/doubled-bytes/case195-195.txt"),
      include_str!("common/data/pairs/doubled-bytes/case196-196.txt"),
      include_str!("common/data/pairs/doubled-bytes/case197-197.txt"),
      include_str!("common/data/pairs/doubled-bytes/case198-198.txt"),
      include_str!("common/data/pairs/doubled-bytes/case199-199.txt"),
      include_str!("common/data/pairs/doubled-bytes/case200-200.txt"),
      include_str!("common/data/pairs/doubled-bytes/case201-201.txt"),
      include_str!("common/data/pairs/doubled-bytes/case20-20.txt"),
      include_str!("common/data/pairs/doubled-bytes/case202-202.txt"),
      include_str!("common/data/pairs/doubled-bytes/case203-203.txt"),
      include_str!("common/data/pairs/doubled-bytes/case204-204.txt"),
      include_str!("common/data/pairs/doubled-bytes/case205-205.txt"),
      include_str!("common/data/pairs/doubled-bytes/case206-206.txt"),
      include_str!("common/data/pairs/doubled-bytes/case207-207.txt"),
      include_str!("common/data/pairs/doubled-bytes/case208-208.txt"),
      include_str!("common/data/pairs/doubled-bytes/case209-209.txt"),
      include_str!("common/data/pairs/doubled-bytes/case210-210.txt"),
      include_str!("common/data/pairs/doubled-bytes/case211-211.txt"),
      include_str!("common/data/pairs/doubled-bytes/case21-21.txt"),
      include_str!("common/data/pairs/doubled-bytes/case212-212.txt"),
      include_str!("common/data/pairs/doubled-bytes/case213-213.txt"),
      include_str!("common/data/pairs/doubled-bytes/case214-214.txt"),
      include_str!("common/data/pairs/doubled-bytes/case215-215.txt"),
      include_str!("common/data/pairs/doubled-bytes/case216-216.txt"),
      include_str!("common/data/pairs/doubled-bytes/case217-217.txt"),
      include_str!("common/data/pairs/doubled-bytes/case218-218.txt"),
      include_str!("common/data/pairs/doubled-bytes/case219-219.txt"),
      include_str!("common/data/pairs/doubled-bytes/case220-220.txt"),
      include_str!("common/data/pairs/doubled-bytes/case221-221.txt"),
      include_str!("common/data/pairs/doubled-bytes/case222-222.txt"),
      include_str!("common/data/pairs/doubled-bytes/case22-22.txt"),
      include_str!("common/data/pairs/doubled-bytes/case223-223.txt"),
      include_str!("common/data/pairs/doubled-bytes/case224-224.txt"),
      include_str!("common/data/pairs/doubled-bytes/case225-225.txt"),
      include_str!("common/data/pairs/doubled-bytes/case226-226.txt"),
      include_str!("common/data/pairs/doubled-bytes/case227-227.txt"),
      include_str!("common/data/pairs/doubled-bytes/case228-228.txt"),
      include_str!("common/data/pairs/doubled-bytes/case229-229.txt"),
      include_str!("common/data/pairs/doubled-bytes/case2-2.txt"),
      include_str!("common/data/pairs/doubled-bytes/case230-230.txt"),
      include_str!("common/data/pairs/doubled-bytes/case231-231.txt"),
      include_str!("common/data/pairs/doubled-bytes/case232-232.txt"),
      include_str!("common/data/pairs/doubled-bytes/case23-23.txt"),
      include_str!("common/data/pairs/doubled-bytes/case233-233.txt"),
      include_str!("common/data/pairs/doubled-bytes/case234-234.txt"),
      include_str!("common/data/pairs/doubled-bytes/case235-235.txt"),
      include_str!("common/data/pairs/doubled-bytes/case236-236.txt"),
      include_str!("common/data/pairs/doubled-bytes/case237-237.txt"),
      include_str!("common/data/pairs/doubled-bytes/case238-238.txt"),
      include_str!("common/data/pairs/doubled-bytes/case239-239.txt"),
      include_str!("common/data/pairs/doubled-bytes/case240-240.txt"),
      include_str!("common/data/pairs/doubled-bytes/case241-241.txt"),
      include_str!("common/data/pairs/doubled-bytes/case242-242.txt"),
      include_str!("common/data/pairs/doubled-bytes/case24-24.txt"),
      include_str!("common/data/pairs/doubled-bytes/case243-243.txt"),
      include_str!("common/data/pairs/doubled-bytes/case244-244.txt"),
      include_str!("common/data/pairs/doubled-bytes/case245-245.txt"),
      include_str!("common/data/pairs/doubled-bytes/case246-246.txt"),
      include_str!("common/data/pairs/doubled-bytes/case247-247.txt"),
      include_str!("common/data/pairs/doubled-bytes/case248-248.txt"),
      include_str!("common/data/pairs/doubled-bytes/case249-249.txt"),
      include_str!("common/data/pairs/doubled-bytes/case250-250.txt"),
      include_str!("common/data/pairs/doubled-bytes/case251-251.txt"),
      include_str!("common/data/pairs/doubled-bytes/case252-252.txt"),
      include_str!("common/data/pairs/doubled-bytes/case25-25.txt"),
      include_str!("common/data/pairs/doubled-bytes/case253-253.txt"),
      include_str!("common/data/pairs/doubled-bytes/case254-254.txt"),
      include_str!("common/data/pairs/doubled-bytes/case255-255.txt"),
      include_str!("common/data/pairs/doubled-bytes/case26-26.txt"),
      include_str!("common/data/pairs/doubled-bytes/case27-27.txt"),
      include_str!("common/data/pairs/doubled-bytes/case28-28.txt"),
      include_str!("common/data/pairs/doubled-bytes/case29-29.txt"),
      include_str!("common/data/pairs/doubled-bytes/case30-30.txt"),
      include_str!("common/data/pairs/doubled-bytes/case31-31.txt"),
      include_str!("common/data/pairs/doubled-bytes/case32-32.txt"),
      include_str!("common/data/pairs/doubled-bytes/case33-33.txt"),
      include_str!("common/data/pairs/doubled-bytes/case3-3.txt"),
      include_str!("common/data/pairs/doubled-bytes/case34-34.txt"),
      include_str!("common/data/pairs/doubled-bytes/case35-35.txt"),
      include_str!("common/data/pairs/doubled-bytes/case36-36.txt"),
      include_str!("common/data/pairs/doubled-bytes/case37-37.txt"),
      include_str!("common/data/pairs/doubled-bytes/case38-38.txt"),
      include_str!("common/data/pairs/doubled-bytes/case39-39.txt"),
      include_str!("common/data/pairs/doubled-bytes/case40-40.txt"),
      include_str!("common/data/pairs/doubled-bytes/case41-41.txt"),
      include_str!("common/data/pairs/doubled-bytes/case42-42.txt"),
      include_str!("common/data/pairs/doubled-bytes/case43-43.txt"),
      include_str!("common/data/pairs/doubled-bytes/case44-44.txt"),
      include_str!("common/data/pairs/doubled-bytes/case4-4.txt"),
      include_str!("common/data/pairs/doubled-bytes/case45-45.txt"),
      include_str!("common/data/pairs/doubled-bytes/case46-46.txt"),
      include_str!("common/data/pairs/doubled-bytes/case47-47.txt"),
      include_str!("common/data/pairs/doubled-bytes/case48-48.txt"),
      include_str!("common/data/pairs/doubled-bytes/case49-49.txt"),
      include_str!("common/data/pairs/doubled-bytes/case50-50.txt"),
      include_str!("common/data/pairs/doubled-bytes/case51-51.txt"),
      include_str!("common/data/pairs/doubled-bytes/case52-52.txt"),
      include_str!("common/data/pairs/doubled-bytes/case53-53.txt"),
      include_str!("common/data/pairs/doubled-bytes/case54-54.txt"),
      include_str!("common/data/pairs/doubled-bytes/case55-55.txt"),
      include_str!("common/data/pairs/doubled-bytes/case5-5.txt"),
      include_str!("common/data/pairs/doubled-bytes/case56-56.txt"),
      include_str!("common/data/pairs/doubled-bytes/case57-57.txt"),
      include_str!("common/data/pairs/doubled-bytes/case58-58.txt"),
      include_str!("common/data/pairs/doubled-bytes/case59-59.txt"),
      include_str!("common/data/pairs/doubled-bytes/case60-60.txt"),
      include_str!("common/data/pairs/doubled-bytes/case61-61.txt"),
      include_str!("common/data/pairs/doubled-bytes/case62-62.txt"),
      include_str!("common/data/pairs/doubled-bytes/case63-63.txt"),
      include_str!("common/data/pairs/doubled-bytes/case64-64.txt"),
      include_str!("common/data/pairs/doubled-bytes/case65-65.txt"),
      include_str!("common/data/pairs/doubled-bytes/case66-66.txt"),
      include_str!("common/data/pairs/doubled-bytes/case6-6.txt"),
      include_str!("common/data/pairs/doubled-bytes/case67-67.txt"),
      include_str!("common/data/pairs/doubled-bytes/case68-68.txt"),
      include_str!("common/data/pairs/doubled-bytes/case69-69.txt"),
      include_str!("common/data/pairs/doubled-bytes/case70-70.txt"),
      include_str!("common/data/pairs/doubled-bytes/case71-71.txt"),
      include_str!("common/data/pairs/doubled-bytes/case72-72.txt"),
      include_str!("common/data/pairs/doubled-bytes/case73-73.txt"),
      include_str!("common/data/pairs/doubled-bytes/case74-74.txt"),
      include_str!("common/data/pairs/doubled-bytes/case75-75.txt"),
      include_str!("common/data/pairs/doubled-bytes/case76-76.txt"),
      include_str!("common/data/pairs/doubled-bytes/case77-77.txt"),
      include_str!("common/data/pairs/doubled-bytes/case7-7.txt"),
      include_str!("common/data/pairs/doubled-bytes/case78-78.txt"),
      include_str!("common/data/pairs/doubled-bytes/case79-79.txt"),
      include_str!("common/data/pairs/doubled-bytes/case80-80.txt"),
      include_str!("common/data/pairs/doubled-bytes/case81-81.txt"),
      include_str!("common/data/pairs/doubled-bytes/case82-82.txt"),
      include_str!("common/data/pairs/doubled-bytes/case83-83.txt"),
      include_str!("common/data/pairs/doubled-bytes/case84-84.txt"),
      include_str!("common/data/pairs/doubled-bytes/case85-85.txt"),
      include_str!("common/data/pairs/doubled-bytes/case86-86.txt"),
      include_str!("common/data/pairs/doubled-bytes/case87-87.txt"),
      include_str!("common/data/pairs/doubled-bytes/case88-88.txt"),
      include_str!("common/data/pairs/doubled-bytes/case8-8.txt"),
      include_str!("common/data/pairs/doubled-bytes/case89-89.txt"),
      include_str!("common/data/pairs/doubled-bytes/case90-90.txt"),
      include_str!("common/data/pairs/doubled-bytes/case91-91.txt"),
      include_str!("common/data/pairs/doubled-bytes/case92-92.txt"),
      include_str!("common/data/pairs/doubled-bytes/case93-93.txt"),
      include_str!("common/data/pairs/doubled-bytes/case94-94.txt"),
      include_str!("common/data/pairs/doubled-bytes/case95-95.txt"),
      include_str!("common/data/pairs/doubled-bytes/case96-96.txt"),
      include_str!("common/data/pairs/doubled-bytes/case97-97.txt"),
      include_str!("common/data/pairs/doubled-bytes/case98-98.txt"),
      include_str!("common/data/pairs/doubled-bytes/case99-99.txt"),
      include_str!("common/data/pairs/doubled-bytes/case9-9.txt")];

#[test]
fn sanity() {
    assert_eq!(BIN.len(), TXT.len());
}

#[test]
fn encode() {
    for i in 0..BIN.len() {
        let input = BIN[i];
        let expected = TXT[i];

        assert_eq!(super::encode(input), expected, "Failed at i = {}", i);
    }
}

#[test]
fn decode() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        assert_eq!(super::decode(input).unwrap(),
                   expected,
                   "Failed at i = {}",
                   i);
    }
}
