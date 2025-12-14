pub trait ToComma {
    fn comma(&self) -> String;
}
impl ToComma for str {
    fn comma(&self) -> String {
        let s = self.as_bytes();
        if s.is_empty() {
            return "".into();
        }
        let p = s.iter().position(|&c| c == b'.').unwrap_or(s.len());
        let (l, r) = s.split_at(p);
        let (sg, nm) = if l.first() == Some(&b'-') {
            (&l[..1], &l[1..])
        } else {
            (&l[..0], l)
        };
        if nm.len() < 4 {
            return self.into();
        }
        let mut v = Vec::with_capacity(self.len() + (nm.len() - 1) / 3);
        v.extend_from_slice(sg);
        let h = (nm.len() - 1) % 3 + 1;
        v.extend_from_slice(&nm[..h]);
        for c in nm[h..].chunks(3) {
            v.push(b',');
            v.extend_from_slice(c)
        }
        v.extend_from_slice(r);
        unsafe { String::from_utf8_unchecked(v) }
    }
}
impl ToComma for String {
    #[inline]
    fn comma(&self) -> String {
        (**self).comma()
    }
}
macro_rules! C{
    (@ $n:expr,$neg:expr)=>{{let(mut n,mut b,mut p)=($n,[0u8;50],50);if n==0{return "0".into()}let mut c=0;while n>0{if c>0&&c%3==0{p-=1;b[p]=b','}p-=1;b[p]=b'0'+(n%10)as u8;n/=10;c+=1}if $neg{p-=1;b[p]=b'-'}unsafe{String::from_utf8_unchecked(b[p..].to_vec())}}};
    (S $($t:ty),*)=>{$(impl ToComma for$t{fn comma(&self)->String{C!(@self.unsigned_abs()as u128,*self<0)}})*};
    (U $($t:ty),*)=>{$(impl ToComma for$t{fn comma(&self)->String{C!(@*self as u128,false)}})*};
}
C!(S i8,i16,i32,i64,i128,isize);
C!(U u8,u16,u32,u64,u128,usize);

impl ToComma for f64 {
    #[inline]
    fn comma(&self) -> String {
        self.to_string().comma()
    }
}
