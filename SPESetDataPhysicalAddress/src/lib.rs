#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Unreachable::*;
use Bit::*;
use u__IMPDEF_boolean::*;
use HaveMTE2Ext::*;
use Zeros::*;
use AArch64_PhysicalTag::*;
use common::*;
pub fn SPESetDataPhysicalAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    addrdesc: ProductTypece7c66ccb2cab13e,
    accdesc: ProductType9878976b5bcce9c9,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_15498: ProductTypeda0231e9dc169f81,
        ga_15477: ProductTypeda0231e9dc169f81,
        ns: bool,
        ga_15478: u32,
        nse: bool,
        pat: u8,
        addrdesc: ProductTypece7c66ccb2cab13e,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        addrdesc,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var addrdesc.3:struct
        let s_0_0: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_0_1: write-var ga#15477 <= s_0_0
        fn_state.ga_15477 = s_0_0;
        // D s_0_2: read-var ga#15477.1:struct
        let s_0_2: u32 = fn_state.ga_15477._1;
        // D s_0_3: write-var ga#15478 <= s_0_2
        fn_state.ga_15478 = s_0_2;
        // C s_0_4: const #1u : u32
        let s_0_4: u32 = 1;
        // D s_0_5: read-var ga#15478:u32
        let s_0_5: u32 = fn_state.ga_15478;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // D s_0_7: not s_0_6
        let s_0_7: bool = !s_0_6;
        // N s_0_8: branch s_0_7 b11 b1
        if s_0_7 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var ns <= s_1_0
        fn_state.ns = s_1_0;
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // D s_1_3: write-var nse <= s_1_2
        fn_state.nse = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveMTE2Ext(s_2_0)
        let s_2_1: bool = HaveMTE2Ext(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b5 b3
        if s_2_1 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var addrdesc.3:struct
        let s_4_0: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_4_1: write-var ga#15498 <= s_4_0
        fn_state.ga_15498 = s_4_0;
        // D s_4_2: read-var ga#15498.0:struct
        let s_4_2: u64 = fn_state.ga_15498._0;
        // C s_4_3: const #13776u : u32
        let s_4_3: u32 = 13776;
        // D s_4_4: read-reg s_4_3:[u64; 32]
        let s_4_4: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // C s_4_5: const #1040u : u32
        let s_4_5: u32 = 1040;
        // D s_4_6: read-reg s_4_5:i64
        let s_4_6: i64 = {
            let value = state.read_register::<i64>(s_4_5 as isize);
            tracer.read_register(s_4_5 as isize, value);
            value
        };
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: read-element s_4_4[s_4_7]
        let s_4_8: u64 = s_4_4[(s_4_7) as usize];
        // C s_4_9: const #55s : i64
        let s_4_9: i64 = 55;
        // C s_4_10: const #0s : i
        let s_4_10: i128 = 0;
        // D s_4_11: cast zx s_4_8 -> bv
        let s_4_11: Bits = Bits::new(s_4_8 as u128, 64u16);
        // C s_4_12: cast zx s_4_9 -> i
        let s_4_12: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_13: cast zx s_4_2 -> bv
        let s_4_13: Bits = Bits::new(s_4_2 as u128, 56u16);
        // C s_4_14: sub s_4_12 s_4_10
        let s_4_14: i128 = ((s_4_12) - (s_4_10));
        // C s_4_15: const #1u : u64
        let s_4_15: u64 = 1;
        // C s_4_16: cast zx s_4_15 -> bv
        let s_4_16: Bits = Bits::new(s_4_15 as u128, 64u16);
        // C s_4_17: lsl s_4_16 s_4_14
        let s_4_17: Bits = s_4_16 << s_4_14;
        // C s_4_18: sub s_4_17 s_4_16
        let s_4_18: Bits = ((s_4_17) - (s_4_16));
        // D s_4_19: and s_4_13 s_4_18
        let s_4_19: Bits = ((s_4_13) & (s_4_18));
        // D s_4_20: lsl s_4_19 s_4_10
        let s_4_20: Bits = s_4_19 << s_4_10;
        // C s_4_21: lsl s_4_18 s_4_10
        let s_4_21: Bits = s_4_18 << s_4_10;
        // C s_4_22: cmpl s_4_21
        let s_4_22: Bits = !s_4_21;
        // D s_4_23: and s_4_11 s_4_22
        let s_4_23: Bits = ((s_4_11) & (s_4_22));
        // D s_4_24: or s_4_23 s_4_20
        let s_4_24: Bits = ((s_4_23) | (s_4_20));
        // D s_4_25: cast reint s_4_24 -> u64
        let s_4_25: u64 = (s_4_24.value() as u64);
        // C s_4_26: const #13776u : u32
        let s_4_26: u32 = 13776;
        // D s_4_27: read-reg s_4_26:[u64; 32]
        let s_4_27: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_4_26 as isize);
            tracer.read_register(s_4_26 as isize, value);
            value
        };
        // C s_4_28: const #1040u : u32
        let s_4_28: u32 = 1040;
        // D s_4_29: read-reg s_4_28:i64
        let s_4_29: i64 = {
            let value = state.read_register::<i64>(s_4_28 as isize);
            tracer.read_register(s_4_28 as isize, value);
            value
        };
        // D s_4_30: cast zx s_4_29 -> i
        let s_4_30: i128 = (i128::try_from(s_4_29).unwrap());
        // D s_4_31: mutate-element s_4_27[s_4_30] <= s_4_25
        let s_4_31: [u64; 32usize] = {
            let mut local = s_4_27.clone();
            local[(s_4_30) as usize] = s_4_25;
            local
        };
        // D s_4_32: cast cvt s_4_31 -> [u64; 0]
        let s_4_32: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_4_31);
        // D s_4_33: cast cvt s_4_32 -> [u64; 32]
        let s_4_33: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_4_32);
            buf
        };
        // C s_4_34: const #13776u : u32
        let s_4_34: u32 = 13776;
        // N s_4_35: write-reg s_4_34 <= s_4_33
        let s_4_35: () = {
            state.write_register::<[u64; 32usize]>(s_4_34 as isize, s_4_33);
            tracer.write_register(s_4_34 as isize, s_4_33);
        };
        // C s_4_36: const #13776u : u32
        let s_4_36: u32 = 13776;
        // D s_4_37: read-reg s_4_36:[u64; 32]
        let s_4_37: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_4_36 as isize);
            tracer.read_register(s_4_36 as isize, value);
            value
        };
        // C s_4_38: const #1040u : u32
        let s_4_38: u32 = 1040;
        // D s_4_39: read-reg s_4_38:i64
        let s_4_39: i64 = {
            let value = state.read_register::<i64>(s_4_38 as isize);
            tracer.read_register(s_4_38 as isize, value);
            value
        };
        // D s_4_40: cast zx s_4_39 -> i
        let s_4_40: i128 = (i128::try_from(s_4_39).unwrap());
        // D s_4_41: read-element s_4_37[s_4_40]
        let s_4_41: u64 = s_4_37[(s_4_40) as usize];
        // D s_4_42: read-var ns:u8
        let s_4_42: bool = fn_state.ns;
        // D s_4_43: call Bit(s_4_42)
        let s_4_43: bool = Bit(state, tracer, s_4_42);
        // C s_4_44: const #63s : i
        let s_4_44: i128 = 63;
        // D s_4_45: cast zx s_4_41 -> bv
        let s_4_45: Bits = Bits::new(s_4_41 as u128, 64u16);
        // C s_4_46: const #1u : u64
        let s_4_46: u64 = 1;
        // D s_4_47: bit-insert s_4_45 s_4_45 s_4_44 s_4_46
        let s_4_47: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_4_46 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_4_45.length(),
            );
            (s_4_45 & mask) | (s_4_45 << s_4_44)
        };
        // D s_4_48: cast reint s_4_47 -> u64
        let s_4_48: u64 = (s_4_47.value() as u64);
        // C s_4_49: const #13776u : u32
        let s_4_49: u32 = 13776;
        // D s_4_50: read-reg s_4_49:[u64; 32]
        let s_4_50: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_4_49 as isize);
            tracer.read_register(s_4_49 as isize, value);
            value
        };
        // C s_4_51: const #1040u : u32
        let s_4_51: u32 = 1040;
        // D s_4_52: read-reg s_4_51:i64
        let s_4_52: i64 = {
            let value = state.read_register::<i64>(s_4_51 as isize);
            tracer.read_register(s_4_51 as isize, value);
            value
        };
        // D s_4_53: cast zx s_4_52 -> i
        let s_4_53: i128 = (i128::try_from(s_4_52).unwrap());
        // D s_4_54: mutate-element s_4_50[s_4_53] <= s_4_48
        let s_4_54: [u64; 32usize] = {
            let mut local = s_4_50.clone();
            local[(s_4_53) as usize] = s_4_48;
            local
        };
        // D s_4_55: cast cvt s_4_54 -> [u64; 0]
        let s_4_55: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_4_54);
        // D s_4_56: cast cvt s_4_55 -> [u64; 32]
        let s_4_56: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_4_55);
            buf
        };
        // C s_4_57: const #13776u : u32
        let s_4_57: u32 = 13776;
        // N s_4_58: write-reg s_4_57 <= s_4_56
        let s_4_58: () = {
            state.write_register::<[u64; 32usize]>(s_4_57 as isize, s_4_56);
            tracer.write_register(s_4_57 as isize, s_4_56);
        };
        // C s_4_59: const #13776u : u32
        let s_4_59: u32 = 13776;
        // D s_4_60: read-reg s_4_59:[u64; 32]
        let s_4_60: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_4_59 as isize);
            tracer.read_register(s_4_59 as isize, value);
            value
        };
        // C s_4_61: const #1040u : u32
        let s_4_61: u32 = 1040;
        // D s_4_62: read-reg s_4_61:i64
        let s_4_62: i64 = {
            let value = state.read_register::<i64>(s_4_61 as isize);
            tracer.read_register(s_4_61 as isize, value);
            value
        };
        // D s_4_63: cast zx s_4_62 -> i
        let s_4_63: i128 = (i128::try_from(s_4_62).unwrap());
        // D s_4_64: read-element s_4_60[s_4_63]
        let s_4_64: u64 = s_4_60[(s_4_63) as usize];
        // D s_4_65: read-var nse:u8
        let s_4_65: bool = fn_state.nse;
        // D s_4_66: call Bit(s_4_65)
        let s_4_66: bool = Bit(state, tracer, s_4_65);
        // C s_4_67: const #60s : i
        let s_4_67: i128 = 60;
        // D s_4_68: cast zx s_4_64 -> bv
        let s_4_68: Bits = Bits::new(s_4_64 as u128, 64u16);
        // C s_4_69: const #1u : u64
        let s_4_69: u64 = 1;
        // D s_4_70: bit-insert s_4_68 s_4_68 s_4_67 s_4_69
        let s_4_70: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_4_69 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_4_68.length(),
            );
            (s_4_68 & mask) | (s_4_68 << s_4_67)
        };
        // D s_4_71: cast reint s_4_70 -> u64
        let s_4_71: u64 = (s_4_70.value() as u64);
        // C s_4_72: const #13776u : u32
        let s_4_72: u32 = 13776;
        // D s_4_73: read-reg s_4_72:[u64; 32]
        let s_4_73: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_4_72 as isize);
            tracer.read_register(s_4_72 as isize, value);
            value
        };
        // C s_4_74: const #1040u : u32
        let s_4_74: u32 = 1040;
        // D s_4_75: read-reg s_4_74:i64
        let s_4_75: i64 = {
            let value = state.read_register::<i64>(s_4_74 as isize);
            tracer.read_register(s_4_74 as isize, value);
            value
        };
        // D s_4_76: cast zx s_4_75 -> i
        let s_4_76: i128 = (i128::try_from(s_4_75).unwrap());
        // D s_4_77: mutate-element s_4_73[s_4_76] <= s_4_71
        let s_4_77: [u64; 32usize] = {
            let mut local = s_4_73.clone();
            local[(s_4_76) as usize] = s_4_71;
            local
        };
        // D s_4_78: cast cvt s_4_77 -> [u64; 0]
        let s_4_78: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_4_77);
        // D s_4_79: cast cvt s_4_78 -> [u64; 32]
        let s_4_79: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_4_78);
            buf
        };
        // C s_4_80: const #13776u : u32
        let s_4_80: u32 = 13776;
        // N s_4_81: write-reg s_4_80 <= s_4_79
        let s_4_81: () = {
            state.write_register::<[u64; 32usize]>(s_4_80 as isize, s_4_79);
            tracer.write_register(s_4_80 as isize, s_4_79);
        };
        // C s_4_82: const #11136u : u32
        let s_4_82: u32 = 11136;
        // D s_4_83: read-reg s_4_82:[u8; 32]
        let s_4_83: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_4_82 as isize);
            tracer.read_register(s_4_82 as isize, value);
            value
        };
        // C s_4_84: const #1040u : u32
        let s_4_84: u32 = 1040;
        // D s_4_85: read-reg s_4_84:i64
        let s_4_85: i64 = {
            let value = state.read_register::<i64>(s_4_84 as isize);
            tracer.read_register(s_4_84 as isize, value);
            value
        };
        // D s_4_86: cast zx s_4_85 -> i
        let s_4_86: i128 = (i128::try_from(s_4_85).unwrap());
        // C s_4_87: const #1u : u8
        let s_4_87: bool = true;
        // D s_4_88: mutate-element s_4_83[s_4_86] <= s_4_87
        let s_4_88: [bool; 32usize] = {
            let mut local = s_4_83.clone();
            local[(s_4_86) as usize] = s_4_87;
            local
        };
        // D s_4_89: cast cvt s_4_88 -> [u8; 0]
        let s_4_89: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_4_88);
        // D s_4_90: cast cvt s_4_89 -> [u8; 32]
        let s_4_90: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_4_89);
            buf
        };
        // C s_4_91: const #11136u : u32
        let s_4_91: u32 = 11136;
        // N s_4_92: write-reg s_4_91 <= s_4_90
        let s_4_92: () = {
            state.write_register::<[bool; 32usize]>(s_4_91 as isize, s_4_90);
            tracer.write_register(s_4_91 as isize, s_4_90);
        };
        // N s_4_93: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var accdesc.28:struct
        let s_5_0: bool = fn_state.accdesc._28;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #"SPE PAT for tag unchecked access zero" : str
        let s_6_0: &'static str = "SPE PAT for tag unchecked access zero";
        // S s_6_1: call __IMPDEF_boolean(s_6_0)
        let s_6_1: bool = u__IMPDEF_boolean(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // N s_6_3: branch s_6_2 b9 b7
        if s_6_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #4s : i
        let s_7_0: i128 = 4;
        // S s_7_1: call Zeros(s_7_0)
        let s_7_1: Bits = Zeros(state, tracer, s_7_0);
        // S s_7_2: cast reint s_7_1 -> u8
        let s_7_2: u8 = (s_7_1.value() as u8);
        // D s_7_3: write-var pat <= s_7_2
        fn_state.pat = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #13776u : u32
        let s_8_0: u32 = 13776;
        // D s_8_1: read-reg s_8_0:[u64; 32]
        let s_8_1: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #1040u : u32
        let s_8_2: u32 = 1040;
        // D s_8_3: read-reg s_8_2:i64
        let s_8_3: i64 = {
            let value = state.read_register::<i64>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: read-element s_8_1[s_8_4]
        let s_8_5: u64 = s_8_1[(s_8_4) as usize];
        // C s_8_6: const #56s : i
        let s_8_6: i128 = 56;
        // D s_8_7: cast zx s_8_5 -> bv
        let s_8_7: Bits = Bits::new(s_8_5 as u128, 64u16);
        // D s_8_8: read-var pat:u8
        let s_8_8: u8 = fn_state.pat;
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 4u16);
        // C s_8_10: const #3s : i
        let s_8_10: i128 = 3;
        // C s_8_11: const #1u : u64
        let s_8_11: u64 = 1;
        // C s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 64u16);
        // C s_8_13: lsl s_8_12 s_8_10
        let s_8_13: Bits = s_8_12 << s_8_10;
        // C s_8_14: sub s_8_13 s_8_12
        let s_8_14: Bits = ((s_8_13) - (s_8_12));
        // D s_8_15: and s_8_9 s_8_14
        let s_8_15: Bits = ((s_8_9) & (s_8_14));
        // D s_8_16: lsl s_8_15 s_8_6
        let s_8_16: Bits = s_8_15 << s_8_6;
        // C s_8_17: lsl s_8_14 s_8_6
        let s_8_17: Bits = s_8_14 << s_8_6;
        // C s_8_18: cmpl s_8_17
        let s_8_18: Bits = !s_8_17;
        // D s_8_19: and s_8_7 s_8_18
        let s_8_19: Bits = ((s_8_7) & (s_8_18));
        // D s_8_20: or s_8_19 s_8_16
        let s_8_20: Bits = ((s_8_19) | (s_8_16));
        // D s_8_21: cast reint s_8_20 -> u64
        let s_8_21: u64 = (s_8_20.value() as u64);
        // C s_8_22: const #13776u : u32
        let s_8_22: u32 = 13776;
        // D s_8_23: read-reg s_8_22:[u64; 32]
        let s_8_23: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_8_22 as isize);
            tracer.read_register(s_8_22 as isize, value);
            value
        };
        // C s_8_24: const #1040u : u32
        let s_8_24: u32 = 1040;
        // D s_8_25: read-reg s_8_24:i64
        let s_8_25: i64 = {
            let value = state.read_register::<i64>(s_8_24 as isize);
            tracer.read_register(s_8_24 as isize, value);
            value
        };
        // D s_8_26: cast zx s_8_25 -> i
        let s_8_26: i128 = (i128::try_from(s_8_25).unwrap());
        // D s_8_27: mutate-element s_8_23[s_8_26] <= s_8_21
        let s_8_27: [u64; 32usize] = {
            let mut local = s_8_23.clone();
            local[(s_8_26) as usize] = s_8_21;
            local
        };
        // D s_8_28: cast cvt s_8_27 -> [u64; 0]
        let s_8_28: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_8_27);
        // D s_8_29: cast cvt s_8_28 -> [u64; 32]
        let s_8_29: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_8_28);
            buf
        };
        // C s_8_30: const #13776u : u32
        let s_8_30: u32 = 13776;
        // N s_8_31: write-reg s_8_30 <= s_8_29
        let s_8_31: () = {
            state.write_register::<[u64; 32usize]>(s_8_30 as isize, s_8_29);
            tracer.write_register(s_8_30 as isize, s_8_29);
        };
        // N s_8_32: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var addrdesc.7:struct
        let s_9_0: u64 = fn_state.addrdesc._7;
        // D s_9_1: call AArch64_PhysicalTag(s_9_0)
        let s_9_1: u8 = AArch64_PhysicalTag(state, tracer, s_9_0);
        // D s_9_2: write-var pat <= s_9_1
        fn_state.pat = s_9_1;
        // N s_9_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #13776u : u32
        let s_10_0: u32 = 13776;
        // D s_10_1: read-reg s_10_0:[u64; 32]
        let s_10_1: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #1040u : u32
        let s_10_2: u32 = 1040;
        // D s_10_3: read-reg s_10_2:i64
        let s_10_3: i64 = {
            let value = state.read_register::<i64>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: read-element s_10_1[s_10_4]
        let s_10_5: u64 = s_10_1[(s_10_4) as usize];
        // C s_10_6: const #1u : u8
        let s_10_6: bool = true;
        // S s_10_7: call Bit(s_10_6)
        let s_10_7: bool = Bit(state, tracer, s_10_6);
        // C s_10_8: const #62s : i
        let s_10_8: i128 = 62;
        // D s_10_9: cast zx s_10_5 -> bv
        let s_10_9: Bits = Bits::new(s_10_5 as u128, 64u16);
        // C s_10_10: const #1u : u64
        let s_10_10: u64 = 1;
        // D s_10_11: bit-insert s_10_9 s_10_9 s_10_8 s_10_10
        let s_10_11: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_10_10 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_10_9.length(),
            );
            (s_10_9 & mask) | (s_10_9 << s_10_8)
        };
        // D s_10_12: cast reint s_10_11 -> u64
        let s_10_12: u64 = (s_10_11.value() as u64);
        // C s_10_13: const #13776u : u32
        let s_10_13: u32 = 13776;
        // D s_10_14: read-reg s_10_13:[u64; 32]
        let s_10_14: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_10_13 as isize);
            tracer.read_register(s_10_13 as isize, value);
            value
        };
        // C s_10_15: const #1040u : u32
        let s_10_15: u32 = 1040;
        // D s_10_16: read-reg s_10_15:i64
        let s_10_16: i64 = {
            let value = state.read_register::<i64>(s_10_15 as isize);
            tracer.read_register(s_10_15 as isize, value);
            value
        };
        // D s_10_17: cast zx s_10_16 -> i
        let s_10_17: i128 = (i128::try_from(s_10_16).unwrap());
        // D s_10_18: mutate-element s_10_14[s_10_17] <= s_10_12
        let s_10_18: [u64; 32usize] = {
            let mut local = s_10_14.clone();
            local[(s_10_17) as usize] = s_10_12;
            local
        };
        // D s_10_19: cast cvt s_10_18 -> [u64; 0]
        let s_10_19: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_10_18);
        // D s_10_20: cast cvt s_10_19 -> [u64; 32]
        let s_10_20: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_10_19);
            buf
        };
        // C s_10_21: const #13776u : u32
        let s_10_21: u32 = 13776;
        // N s_10_22: write-reg s_10_21 <= s_10_20
        let s_10_22: () = {
            state.write_register::<[u64; 32usize]>(s_10_21 as isize, s_10_20);
            tracer.write_register(s_10_21 as isize, s_10_20);
        };
        // D s_10_23: read-var addrdesc.7:struct
        let s_10_23: u64 = fn_state.addrdesc._7;
        // D s_10_24: call AArch64_PhysicalTag(s_10_23)
        let s_10_24: u8 = AArch64_PhysicalTag(state, tracer, s_10_23);
        // D s_10_25: write-var pat <= s_10_24
        fn_state.pat = s_10_24;
        // N s_10_26: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // D s_11_1: read-var ga#15478:u32
        let s_11_1: u32 = fn_state.ga_15478;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var ns <= s_12_0
        fn_state.ns = s_12_0;
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // D s_12_3: write-var nse <= s_12_2
        fn_state.nse = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #3u : u32
        let s_13_0: u32 = 3;
        // D s_13_1: read-var ga#15478:u32
        let s_13_1: u32 = fn_state.ga_15478;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var ns <= s_14_0
        fn_state.ns = s_14_0;
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // D s_14_3: write-var nse <= s_14_2
        fn_state.nse = s_14_2;
        // N s_14_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Unreachable(s_15_0)
        let s_15_1: () = Unreachable(state, tracer, s_15_0);
        // N s_15_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
