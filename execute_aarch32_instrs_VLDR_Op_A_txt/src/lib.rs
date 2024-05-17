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
use MemA_read::*;
use CheckVFPEnabled::*;
use R_read::*;
use PC_read__1::*;
use Align_bits::*;
use BigEndian::*;
use D_set::*;
use Zeros::*;
use S_set::*;
use common::*;
pub fn execute_aarch32_instrs_VLDR_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d: i64,
    esize: i64,
    imm32: u32,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        word1shadow_7491: u32,
        ga_355247: u64,
        base: u32,
        address: u32,
        gs_898026: Bits,
        gs_898037: Bits,
        gs_898041: Bits,
        word2shadow_7492: u32,
        gs_898033: Bits,
        ga_355242: u16,
        add: bool,
        d: i64,
        esize: i64,
        imm32: u32,
        n: i64,
    }
    let fn_state = FunctionState {
        add,
        d,
        esize,
        imm32,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #15s : i
        let s_1_0: i128 = 15;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b20 b2
        if s_1_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call R_read(s_2_1)
        let s_2_2: u32 = R_read(state, tracer, s_2_1);
        // D s_2_3: write-var base <= s_2_2
        fn_state.base = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var add:u8
        let s_3_0: bool = fn_state.add;
        // N s_3_1: branch s_3_0 b19 b4
        if s_3_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var base:u32
        let s_4_0: u32 = fn_state.base;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 32u16);
        // D s_4_2: read-var imm32:u32
        let s_4_2: u32 = fn_state.imm32;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 32u16);
        // D s_4_4: sub s_4_1 s_4_3
        let s_4_4: Bits = ((s_4_1) - (s_4_3));
        // D s_4_5: cast reint s_4_4 -> u32
        let s_4_5: u32 = (s_4_4.value() as u32);
        // D s_4_6: write-var address <= s_4_5
        fn_state.address = s_4_5;
        // N s_4_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // C s_5_1: const #16s : i
        let s_5_1: i128 = 16;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b8 b6
        if s_5_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16s : i
        let s_6_0: i128 = 16;
        // S s_6_1: call Zeros(s_6_0)
        let s_6_1: Bits = Zeros(state, tracer, s_6_0);
        // S s_6_2: cast reint s_6_1 -> u16
        let s_6_2: u16 = (s_6_1.value() as u16);
        // D s_6_3: write-var ga#355242 <= s_6_2
        fn_state.ga_355242 = s_6_2;
        // C s_6_4: const #2s : i64
        let s_6_4: i64 = 2;
        // D s_6_5: read-var address:u32
        let s_6_5: u32 = fn_state.address;
        // D s_6_6: call MemA_read(s_6_5, s_6_4)
        let s_6_6: Bits = MemA_read(state, tracer, s_6_5, s_6_4);
        // D s_6_7: write-var gs#898026 <= s_6_6
        fn_state.gs_898026 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#898026:bv
        let s_7_0: Bits = fn_state.gs_898026;
        // D s_7_1: cast reint s_7_0 -> u16
        let s_7_1: u16 = (s_7_0.value() as u16);
        // D s_7_2: read-var ga#355242:u16
        let s_7_2: u16 = fn_state.ga_355242;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 16u16);
        // D s_7_4: cast zx s_7_1 -> bv
        let s_7_4: Bits = Bits::new(s_7_1 as u128, 16u16);
        // D s_7_5: cast reint s_7_3 -> u128
        let s_7_5: u128 = (s_7_3.value() as u128);
        // D s_7_6: size-of s_7_3
        let s_7_6: u16 = s_7_3.length();
        // D s_7_7: cast reint s_7_4 -> u128
        let s_7_7: u128 = (s_7_4.value() as u128);
        // D s_7_8: size-of s_7_4
        let s_7_8: u16 = s_7_4.length();
        // D s_7_9: lsl s_7_5 s_7_8
        let s_7_9: u128 = s_7_5 << s_7_8;
        // D s_7_10: or s_7_9 s_7_7
        let s_7_10: u128 = ((s_7_9) | (s_7_7));
        // D s_7_11: add s_7_6 s_7_8
        let s_7_11: u16 = (s_7_6 + s_7_8);
        // D s_7_12: create-bits s_7_10 s_7_11
        let s_7_12: Bits = Bits::new(s_7_10, s_7_11);
        // D s_7_13: cast reint s_7_12 -> u32
        let s_7_13: u32 = (s_7_12.value() as u32);
        // D s_7_14: read-var d:i64
        let s_7_14: i64 = fn_state.d;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: call S_set(s_7_15, s_7_13)
        let s_7_16: () = S_set(state, tracer, s_7_15, s_7_13);
        // N s_7_17: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // C s_8_1: const #32s : i
        let s_8_1: i128 = 32;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b11 b9
        if s_8_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #4s : i64
        let s_9_0: i64 = 4;
        // D s_9_1: read-var address:u32
        let s_9_1: u32 = fn_state.address;
        // D s_9_2: call MemA_read(s_9_1, s_9_0)
        let s_9_2: Bits = MemA_read(state, tracer, s_9_1, s_9_0);
        // D s_9_3: write-var gs#898033 <= s_9_2
        fn_state.gs_898033 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#898033:bv
        let s_10_0: Bits = fn_state.gs_898033;
        // D s_10_1: cast reint s_10_0 -> u32
        let s_10_1: u32 = (s_10_0.value() as u32);
        // D s_10_2: read-var d:i64
        let s_10_2: i64 = fn_state.d;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: call S_set(s_10_3, s_10_1)
        let s_10_4: () = S_set(state, tracer, s_10_3, s_10_1);
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esize:i64
        let s_11_0: i64 = fn_state.esize;
        // C s_11_1: const #64s : i
        let s_11_1: i128 = 64;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b18 b12
        if s_11_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #4s : i64
        let s_12_0: i64 = 4;
        // D s_12_1: read-var address:u32
        let s_12_1: u32 = fn_state.address;
        // D s_12_2: call MemA_read(s_12_1, s_12_0)
        let s_12_2: Bits = MemA_read(state, tracer, s_12_1, s_12_0);
        // D s_12_3: write-var gs#898037 <= s_12_2
        fn_state.gs_898037 = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#898037:bv
        let s_13_0: Bits = fn_state.gs_898037;
        // D s_13_1: cast reint s_13_0 -> u32
        let s_13_1: u32 = (s_13_0.value() as u32);
        // D s_13_2: write-var word1shadow#7491 <= s_13_1
        fn_state.word1shadow_7491 = s_13_1;
        // C s_13_3: const #4s : i
        let s_13_3: i128 = 4;
        // D s_13_4: read-var address:u32
        let s_13_4: u32 = fn_state.address;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 32u16);
        // C s_13_6: cast cvt s_13_3 -> bv
        let s_13_6: Bits = Bits::new(s_13_3 as u128, 128);
        // D s_13_7: add s_13_5 s_13_6
        let s_13_7: Bits = (s_13_5 + s_13_6);
        // D s_13_8: cast reint s_13_7 -> u32
        let s_13_8: u32 = (s_13_7.value() as u32);
        // C s_13_9: const #4s : i64
        let s_13_9: i64 = 4;
        // D s_13_10: call MemA_read(s_13_8, s_13_9)
        let s_13_10: Bits = MemA_read(state, tracer, s_13_8, s_13_9);
        // D s_13_11: write-var gs#898041 <= s_13_10
        fn_state.gs_898041 = s_13_10;
        // N s_13_12: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#898041:bv
        let s_14_0: Bits = fn_state.gs_898041;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // D s_14_2: write-var word2shadow#7492 <= s_14_1
        fn_state.word2shadow_7492 = s_14_1;
        // C s_14_3: const #2u : u32
        let s_14_3: u32 = 2;
        // S s_14_4: call BigEndian(s_14_3)
        let s_14_4: bool = BigEndian(state, tracer, s_14_3);
        // N s_14_5: branch s_14_4 b17 b15
        if s_14_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var word2shadow#7492:u32
        let s_15_0: u32 = fn_state.word2shadow_7492;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 32u16);
        // D s_15_2: read-var word1shadow#7491:u32
        let s_15_2: u32 = fn_state.word1shadow_7491;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 32u16);
        // D s_15_4: cast reint s_15_1 -> u128
        let s_15_4: u128 = (s_15_1.value() as u128);
        // D s_15_5: size-of s_15_1
        let s_15_5: u16 = s_15_1.length();
        // D s_15_6: cast reint s_15_3 -> u128
        let s_15_6: u128 = (s_15_3.value() as u128);
        // D s_15_7: size-of s_15_3
        let s_15_7: u16 = s_15_3.length();
        // D s_15_8: lsl s_15_4 s_15_7
        let s_15_8: u128 = s_15_4 << s_15_7;
        // D s_15_9: or s_15_8 s_15_6
        let s_15_9: u128 = ((s_15_8) | (s_15_6));
        // D s_15_10: add s_15_5 s_15_7
        let s_15_10: u16 = (s_15_5 + s_15_7);
        // D s_15_11: create-bits s_15_9 s_15_10
        let s_15_11: Bits = Bits::new(s_15_9, s_15_10);
        // D s_15_12: cast reint s_15_11 -> u64
        let s_15_12: u64 = (s_15_11.value() as u64);
        // D s_15_13: write-var ga#355247 <= s_15_12
        fn_state.ga_355247 = s_15_12;
        // N s_15_14: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var d:i64
        let s_16_0: i64 = fn_state.d;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var ga#355247:u64
        let s_16_2: u64 = fn_state.ga_355247;
        // D s_16_3: call D_set(s_16_1, s_16_2)
        let s_16_3: () = D_set(state, tracer, s_16_1, s_16_2);
        // N s_16_4: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var word1shadow#7491:u32
        let s_17_0: u32 = fn_state.word1shadow_7491;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 32u16);
        // D s_17_2: read-var word2shadow#7492:u32
        let s_17_2: u32 = fn_state.word2shadow_7492;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 32u16);
        // D s_17_4: cast reint s_17_1 -> u128
        let s_17_4: u128 = (s_17_1.value() as u128);
        // D s_17_5: size-of s_17_1
        let s_17_5: u16 = s_17_1.length();
        // D s_17_6: cast reint s_17_3 -> u128
        let s_17_6: u128 = (s_17_3.value() as u128);
        // D s_17_7: size-of s_17_3
        let s_17_7: u16 = s_17_3.length();
        // D s_17_8: lsl s_17_4 s_17_7
        let s_17_8: u128 = s_17_4 << s_17_7;
        // D s_17_9: or s_17_8 s_17_6
        let s_17_9: u128 = ((s_17_8) | (s_17_6));
        // D s_17_10: add s_17_5 s_17_7
        let s_17_10: u16 = (s_17_5 + s_17_7);
        // D s_17_11: create-bits s_17_9 s_17_10
        let s_17_11: Bits = Bits::new(s_17_9, s_17_10);
        // D s_17_12: cast reint s_17_11 -> u64
        let s_17_12: u64 = (s_17_11.value() as u64);
        // D s_17_13: write-var ga#355247 <= s_17_12
        fn_state.ga_355247 = s_17_12;
        // N s_17_14: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var base:u32
        let s_19_0: u32 = fn_state.base;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 32u16);
        // D s_19_2: read-var imm32:u32
        let s_19_2: u32 = fn_state.imm32;
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 32u16);
        // D s_19_4: add s_19_1 s_19_3
        let s_19_4: Bits = (s_19_1 + s_19_3);
        // D s_19_5: cast reint s_19_4 -> u32
        let s_19_5: u32 = (s_19_4.value() as u32);
        // D s_19_6: write-var address <= s_19_5
        fn_state.address = s_19_5;
        // N s_19_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call PC_read__1(s_20_0)
        let s_20_1: u32 = PC_read__1(state, tracer, s_20_0);
        // C s_20_2: const #4s : i
        let s_20_2: i128 = 4;
        // S s_20_3: cast zx s_20_1 -> bv
        let s_20_3: Bits = Bits::new(s_20_1 as u128, 32u16);
        // S s_20_4: call Align_bits(s_20_3, s_20_2)
        let s_20_4: Bits = Align_bits(state, tracer, s_20_3, s_20_2);
        // S s_20_5: cast reint s_20_4 -> u32
        let s_20_5: u32 = (s_20_4.value() as u32);
        // D s_20_6: write-var base <= s_20_5
        fn_state.base = s_20_5;
        // N s_20_7: jump b3
        return block_3(state, tracer, fn_state);
    }
}
