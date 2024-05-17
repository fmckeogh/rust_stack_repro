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
use TTBR1_EL1_read::*;
use TTBR0_EL2_read::*;
use AArch64_GetVARange::*;
use TTBR0_EL1_read::*;
use common::*;
pub fn AArch64_S1TTBR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    va: u64,
) -> u128 {
    #[derive(Default)]
    struct FunctionState {
        ga_13236: ProductType782ac6922b48c20d,
        ga_13242: ProductType782ac6922b48c20d,
        ga_13245: ProductType782ac6922b48c20d,
        ga_13248: u128,
        return_value: u128,
        ga_13232: ProductType782ac6922b48c20d,
        varange: u32,
        regime: u32,
        va: u64,
    }
    let fn_state = FunctionState {
        regime,
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // D s_0_0: read-var va:u64
        let s_0_0: u64 = fn_state.va;
        // D s_0_1: call AArch64_GetVARange(s_0_0)
        let s_0_1: u32 = AArch64_GetVARange(state, tracer, s_0_0);
        // D s_0_2: write-var varange <= s_0_1
        fn_state.varange = s_0_1;
        // C s_0_3: const #0u : u32
        let s_0_3: u32 = 0;
        // D s_0_4: read-var regime:u32
        let s_0_4: u32 = fn_state.regime;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // D s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_1_0: const #23176u : u32
        let s_1_0: u32 = 23176;
        // D s_1_1: read-reg s_1_0:u64
        let s_1_1: u64 = {
            let value = state.read_register::<u64>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 64u16);
        // D s_1_4: bits-cast zx s_1_3 -> bv length s_1_2
        let s_1_4: Bits = s_1_3.zero_extend(s_1_2);
        // D s_1_5: cast reint s_1_4 -> u128
        let s_1_5: u128 = (s_1_4.value() as u128);
        // D s_1_6: write-var return_value <= s_1_5
        fn_state.return_value = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // D s_2_0: read-var return_value:u128
        let s_2_0: u128 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_3_0: const #2u : u32
        let s_3_0: u32 = 2;
        // D s_3_1: read-var regime:u32
        let s_3_1: u32 = fn_state.regime;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call TTBR0_EL2_read(s_4_0)
        let s_4_1: ProductType782ac6922b48c20d = TTBR0_EL2_read(state, tracer, s_4_0);
        // D s_4_2: write-var ga#13232 <= s_4_1
        fn_state.ga_13232 = s_4_1;
        // D s_4_3: read-var ga#13232.0:struct
        let s_4_3: u128 = fn_state.ga_13232._0;
        // C s_4_4: const #128s : i
        let s_4_4: i128 = 128;
        // D s_4_5: cast zx s_4_3 -> bv
        let s_4_5: Bits = Bits::new(s_4_3 as u128, 128u16);
        // D s_4_6: bits-cast zx s_4_5 -> bv length s_4_4
        let s_4_6: Bits = s_4_5.zero_extend(s_4_4);
        // D s_4_7: cast reint s_4_6 -> u128
        let s_4_7: u128 = (s_4_6.value() as u128);
        // D s_4_8: write-var return_value <= s_4_7
        fn_state.return_value = s_4_7;
        // N s_4_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_5_0: const #3u : u32
        let s_5_0: u32 = 3;
        // D s_5_1: read-var regime:u32
        let s_5_1: u32 = fn_state.regime;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b9 b6
        if s_5_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // D s_6_0: read-var varange:u32
        let s_6_0: u32 = fn_state.varange;
        // C s_6_1: const #0u : u32
        let s_6_1: u32 = 0;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_7_0: const #18432u : u32
        let s_7_0: u32 = 18432;
        // D s_7_1: read-reg s_7_0:u128
        let s_7_1: u128 = {
            let value = state.read_register::<u128>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #128s : i
        let s_7_2: i128 = 128;
        // D s_7_3: cast zx s_7_1 -> bv
        let s_7_3: Bits = Bits::new(s_7_1 as u128, 128u16);
        // D s_7_4: bits-cast zx s_7_3 -> bv length s_7_2
        let s_7_4: Bits = s_7_3.zero_extend(s_7_2);
        // D s_7_5: cast reint s_7_4 -> u128
        let s_7_5: u128 = (s_7_4.value() as u128);
        // D s_7_6: write-var return_value <= s_7_5
        fn_state.return_value = s_7_5;
        // N s_7_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call TTBR0_EL2_read(s_8_0)
        let s_8_1: ProductType782ac6922b48c20d = TTBR0_EL2_read(state, tracer, s_8_0);
        // D s_8_2: write-var ga#13236 <= s_8_1
        fn_state.ga_13236 = s_8_1;
        // D s_8_3: read-var ga#13236.0:struct
        let s_8_3: u128 = fn_state.ga_13236._0;
        // C s_8_4: const #128s : i
        let s_8_4: i128 = 128;
        // D s_8_5: cast zx s_8_3 -> bv
        let s_8_5: Bits = Bits::new(s_8_3 as u128, 128u16);
        // D s_8_6: bits-cast zx s_8_5 -> bv length s_8_4
        let s_8_6: Bits = s_8_5.zero_extend(s_8_4);
        // D s_8_7: cast reint s_8_6 -> u128
        let s_8_7: u128 = (s_8_6.value() as u128);
        // D s_8_8: write-var return_value <= s_8_7
        fn_state.return_value = s_8_7;
        // N s_8_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_9_0: const #4u : u32
        let s_9_0: u32 = 4;
        // D s_9_1: read-var regime:u32
        let s_9_1: u32 = fn_state.regime;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b13 b10
        if s_9_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // D s_10_0: read-var varange:u32
        let s_10_0: u32 = fn_state.varange;
        // C s_10_1: const #0u : u32
        let s_10_1: u32 = 0;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b12 b11
        if s_10_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call TTBR1_EL1_read(s_11_0)
        let s_11_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#13245 <= s_11_1
        fn_state.ga_13245 = s_11_1;
        // D s_11_3: read-var ga#13245.0:struct
        let s_11_3: u128 = fn_state.ga_13245._0;
        // C s_11_4: const #128s : i
        let s_11_4: i128 = 128;
        // D s_11_5: cast zx s_11_3 -> bv
        let s_11_5: Bits = Bits::new(s_11_3 as u128, 128u16);
        // D s_11_6: bits-cast zx s_11_5 -> bv length s_11_4
        let s_11_6: Bits = s_11_5.zero_extend(s_11_4);
        // D s_11_7: cast reint s_11_6 -> u128
        let s_11_7: u128 = (s_11_6.value() as u128);
        // D s_11_8: write-var return_value <= s_11_7
        fn_state.return_value = s_11_7;
        // N s_11_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call TTBR0_EL1_read(s_12_0)
        let s_12_1: ProductType782ac6922b48c20d = TTBR0_EL1_read(state, tracer, s_12_0);
        // D s_12_2: write-var ga#13242 <= s_12_1
        fn_state.ga_13242 = s_12_1;
        // D s_12_3: read-var ga#13242.0:struct
        let s_12_3: u128 = fn_state.ga_13242._0;
        // C s_12_4: const #128s : i
        let s_12_4: i128 = 128;
        // D s_12_5: cast zx s_12_3 -> bv
        let s_12_5: Bits = Bits::new(s_12_3 as u128, 128u16);
        // D s_12_6: bits-cast zx s_12_5 -> bv length s_12_4
        let s_12_6: Bits = s_12_5.zero_extend(s_12_4);
        // D s_12_7: cast reint s_12_6 -> u128
        let s_12_7: u128 = (s_12_6.value() as u128);
        // D s_12_8: write-var return_value <= s_12_7
        fn_state.return_value = s_12_7;
        // N s_12_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // D s_13_0: read-var ga#13248:u128
        let s_13_0: u128 = fn_state.ga_13248;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
