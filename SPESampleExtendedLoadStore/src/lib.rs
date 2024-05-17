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
use common::*;
pub fn SPESampleExtendedLoadStore<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ar: bool,
    excl: bool,
    at: bool,
    is_load: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ldst: bool,
        ar: bool,
        excl: bool,
        at: bool,
        is_load: bool,
    }
    let fn_state = FunctionState {
        ar,
        excl,
        at,
        is_load,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: u8 = 1;
        // C s_0_1: const #17136u : u32
        let s_0_1: u32 = 17136;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<u8>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // D s_0_3: read-var is_load:u8
        let s_0_3: bool = fn_state.is_load;
        // N s_0_4: branch s_0_3 b7 b1
        if s_0_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var ldst <= s_1_0
        fn_state.ldst = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: u8 = 0;
        // C s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 3u16);
        // D s_2_2: read-var ar:u8
        let s_2_2: bool = fn_state.ar;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // D s_2_14: read-var excl:u8
        let s_2_14: bool = fn_state.excl;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u8
        let s_2_24: u8 = (s_2_23.value() as u8);
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 5u16);
        // D s_2_26: read-var at:u8
        let s_2_26: bool = fn_state.at;
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 1u16);
        // D s_2_28: cast reint s_2_25 -> u128
        let s_2_28: u128 = (s_2_25.value() as u128);
        // D s_2_29: size-of s_2_25
        let s_2_29: u16 = s_2_25.length();
        // D s_2_30: cast reint s_2_27 -> u128
        let s_2_30: u128 = (s_2_27.value() as u128);
        // D s_2_31: size-of s_2_27
        let s_2_31: u16 = s_2_27.length();
        // D s_2_32: lsl s_2_28 s_2_31
        let s_2_32: u128 = s_2_28 << s_2_31;
        // D s_2_33: or s_2_32 s_2_30
        let s_2_33: u128 = ((s_2_32) | (s_2_30));
        // D s_2_34: add s_2_29 s_2_31
        let s_2_34: u16 = (s_2_29 + s_2_31);
        // D s_2_35: create-bits s_2_33 s_2_34
        let s_2_35: Bits = Bits::new(s_2_33, s_2_34);
        // D s_2_36: cast reint s_2_35 -> u8
        let s_2_36: u8 = (s_2_35.value() as u8);
        // D s_2_37: cast zx s_2_36 -> bv
        let s_2_37: Bits = Bits::new(s_2_36 as u128, 6u16);
        // C s_2_38: const #1u : u8
        let s_2_38: bool = true;
        // C s_2_39: cast zx s_2_38 -> bv
        let s_2_39: Bits = Bits::new(s_2_38 as u128, 1u16);
        // D s_2_40: cast reint s_2_37 -> u128
        let s_2_40: u128 = (s_2_37.value() as u128);
        // D s_2_41: size-of s_2_37
        let s_2_41: u16 = s_2_37.length();
        // C s_2_42: cast reint s_2_39 -> u128
        let s_2_42: u128 = (s_2_39.value() as u128);
        // D s_2_43: size-of s_2_39
        let s_2_43: u16 = s_2_39.length();
        // D s_2_44: lsl s_2_40 s_2_43
        let s_2_44: u128 = s_2_40 << s_2_43;
        // D s_2_45: or s_2_44 s_2_42
        let s_2_45: u128 = ((s_2_44) | (s_2_42));
        // D s_2_46: add s_2_41 s_2_43
        let s_2_46: u16 = (s_2_41 + s_2_43);
        // D s_2_47: create-bits s_2_45 s_2_46
        let s_2_47: Bits = Bits::new(s_2_45, s_2_46);
        // D s_2_48: cast reint s_2_47 -> u8
        let s_2_48: u8 = (s_2_47.value() as u8);
        // D s_2_49: cast zx s_2_48 -> bv
        let s_2_49: Bits = Bits::new(s_2_48 as u128, 7u16);
        // D s_2_50: read-var ldst:u8
        let s_2_50: bool = fn_state.ldst;
        // D s_2_51: cast zx s_2_50 -> bv
        let s_2_51: Bits = Bits::new(s_2_50 as u128, 1u16);
        // D s_2_52: cast reint s_2_49 -> u128
        let s_2_52: u128 = (s_2_49.value() as u128);
        // D s_2_53: size-of s_2_49
        let s_2_53: u16 = s_2_49.length();
        // D s_2_54: cast reint s_2_51 -> u128
        let s_2_54: u128 = (s_2_51.value() as u128);
        // D s_2_55: size-of s_2_51
        let s_2_55: u16 = s_2_51.length();
        // D s_2_56: lsl s_2_52 s_2_55
        let s_2_56: u128 = s_2_52 << s_2_55;
        // D s_2_57: or s_2_56 s_2_54
        let s_2_57: u128 = ((s_2_56) | (s_2_54));
        // D s_2_58: add s_2_53 s_2_55
        let s_2_58: u16 = (s_2_53 + s_2_55);
        // D s_2_59: create-bits s_2_57 s_2_58
        let s_2_59: Bits = Bits::new(s_2_57, s_2_58);
        // D s_2_60: cast reint s_2_59 -> u8
        let s_2_60: u8 = (s_2_59.value() as u8);
        // C s_2_61: const #13528u : u32
        let s_2_61: u32 = 13528;
        // N s_2_62: write-reg s_2_61 <= s_2_60
        let s_2_62: () = {
            state.write_register::<u8>(s_2_61 as isize, s_2_60);
            tracer.write_register(s_2_61 as isize, s_2_60);
        };
        // C s_2_63: const #1u : u8
        let s_2_63: bool = true;
        // C s_2_64: const #11528u : u32
        let s_2_64: u32 = 11528;
        // N s_2_65: write-reg s_2_64 <= s_2_63
        let s_2_65: () = {
            state.write_register::<bool>(s_2_64 as isize, s_2_63);
            tracer.write_register(s_2_64 as isize, s_2_63);
        };
        // D s_2_66: read-var is_load:u8
        let s_2_66: bool = fn_state.is_load;
        // N s_2_67: branch s_2_66 b4 b3
        if s_2_66 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u32
        let s_3_0: u32 = 1;
        // C s_3_1: const #19040u : u32
        let s_3_1: u32 = 19040;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<u32>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // N s_3_3: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var at:u8
        let s_4_0: bool = fn_state.at;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u32
        let s_5_0: u32 = 0;
        // C s_5_1: const #19040u : u32
        let s_5_1: u32 = 19040;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<u32>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2u : u32
        let s_6_0: u32 = 2;
        // C s_6_1: const #19040u : u32
        let s_6_1: u32 = 19040;
        // N s_6_2: write-reg s_6_1 <= s_6_0
        let s_6_2: () = {
            state.write_register::<u32>(s_6_1 as isize, s_6_0);
            tracer.write_register(s_6_1 as isize, s_6_0);
        };
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var ldst <= s_7_0
        fn_state.ldst = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
