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
use HaveMTEStoreOnlyExt::*;
use UsingAArch32::*;
use EffectiveTCMA::*;
use StoreOnlyTagCheckingEnabled::*;
use EffectiveTBI::*;
use EffectiveMTX::*;
use AArch64_AllocationTagAccessIsEnabled::*;
use common::*;
pub fn AArch64_AccessIsTagChecked<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddr: u64,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_15839: bool,
        return_value: bool,
        gs_15844: bool,
        gs_15845: bool,
        gs_15847: bool,
        gs_15846: bool,
        vaddr: u64,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        vaddr,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var accdesc.28:struct
        let s_0_0: bool = fn_state.accdesc._28;
        // N s_0_1: assert s_0_0
        let s_0_1: () = assert!(s_0_0);
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call UsingAArch32(s_0_2)
        let s_0_3: bool = UsingAArch32(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b28 b1
        if s_0_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: read-var vaddr:u64
        let s_1_2: u64 = fn_state.vaddr;
        // C s_1_3: const #0u : u8
        let s_1_3: bool = false;
        // D s_1_4: call EffectiveMTX(s_1_2, s_1_3, s_1_1)
        let s_1_4: bool = EffectiveMTX(state, tracer, s_1_2, s_1_3, s_1_1);
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // C s_1_6: const #0u : u8
        let s_1_6: bool = false;
        // C s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 1u16);
        // D s_1_8: cmp-eq s_1_5 s_1_7
        let s_1_8: bool = ((s_1_5) == (s_1_7));
        // N s_1_9: branch s_1_8 b27 b2
        if s_1_8 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#15839 <= s_2_0
        fn_state.gs_15839 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#15839:u8
        let s_3_0: bool = fn_state.gs_15839;
        // N s_3_1: branch s_3_0 b26 b4
        if s_3_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #16975u : u32
        let s_4_0: u32 = 16975;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: read-var vaddr:u64
        let s_4_2: u64 = fn_state.vaddr;
        // D s_4_3: call EffectiveTCMA(s_4_2, s_4_1)
        let s_4_3: bool = EffectiveTCMA(state, tracer, s_4_2, s_4_1);
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 1u16);
        // C s_4_5: const #1u : u8
        let s_4_5: bool = true;
        // C s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 1u16);
        // D s_4_7: cmp-eq s_4_4 s_4_6
        let s_4_7: bool = ((s_4_4) == (s_4_6));
        // N s_4_8: branch s_4_7 b22 b5
        if s_4_7 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#15845 <= s_5_0
        fn_state.gs_15845 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#15845:u8
        let s_6_0: bool = fn_state.gs_15845;
        // N s_6_1: branch s_6_0 b21 b7
        if s_6_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var accdesc.8:struct
        let s_7_0: u8 = fn_state.accdesc._8;
        // D s_7_1: call AArch64_AllocationTagAccessIsEnabled(s_7_0)
        let s_7_1: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_7_0);
        // D s_7_2: not s_7_1
        let s_7_2: bool = !s_7_1;
        // N s_7_3: branch s_7_2 b20 b8
        if s_7_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #16994u : u32
        let s_8_0: u32 = 16994;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: bool = {
            let value = state.read_register::<bool>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 1u16);
        // C s_8_3: const #1u : u8
        let s_8_3: bool = true;
        // C s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 1u16);
        // D s_8_5: cmp-eq s_8_2 s_8_4
        let s_8_5: bool = ((s_8_2) == (s_8_4));
        // N s_8_6: branch s_8_5 b19 b9
        if s_8_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HaveMTEStoreOnlyExt(s_9_0)
        let s_9_1: bool = HaveMTEStoreOnlyExt(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b18 b10
        if s_9_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#15846 <= s_10_0
        fn_state.gs_15846 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#15846:u8
        let s_11_0: bool = fn_state.gs_15846;
        // N s_11_1: branch s_11_0 b17 b12
        if s_11_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#15847 <= s_12_0
        fn_state.gs_15847 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#15847:u8
        let s_13_0: bool = fn_state.gs_15847;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var return_value:u8
        let s_15_0: bool = fn_state.return_value;
        // N s_15_1: return s_15_0
        return s_15_0;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var return_value <= s_16_0
        fn_state.return_value = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var accdesc.8:struct
        let s_17_0: u8 = fn_state.accdesc._8;
        // D s_17_1: call StoreOnlyTagCheckingEnabled(s_17_0)
        let s_17_1: bool = StoreOnlyTagCheckingEnabled(state, tracer, s_17_0);
        // D s_17_2: write-var gs#15847 <= s_17_1
        fn_state.gs_15847 = s_17_1;
        // N s_17_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var accdesc.32:struct
        let s_18_0: bool = fn_state.accdesc._32;
        // D s_18_1: not s_18_0
        let s_18_1: bool = !s_18_0;
        // D s_18_2: write-var gs#15846 <= s_18_1
        fn_state.gs_15846 = s_18_1;
        // N s_18_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #55s : i
        let s_22_0: i128 = 55;
        // D s_22_1: read-var vaddr:u64
        let s_22_1: u64 = fn_state.vaddr;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 64u16);
        // C s_22_3: const #1s : i64
        let s_22_3: i64 = 1;
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #4s : i
        let s_22_5: i128 = 4;
        // C s_22_6: add s_22_5 s_22_4
        let s_22_6: i128 = (s_22_5 + s_22_4);
        // D s_22_7: bit-extract s_22_2 s_22_0 s_22_6
        let s_22_7: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_6).unwrap(),
        ));
        // D s_22_8: cast reint s_22_7 -> u8
        let s_22_8: u8 = (s_22_7.value() as u8);
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 5u16);
        // C s_22_10: const #0u : u8
        let s_22_10: u8 = 0;
        // C s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 5u16);
        // D s_22_12: cmp-eq s_22_9 s_22_11
        let s_22_12: bool = ((s_22_9) == (s_22_11));
        // N s_22_13: branch s_22_12 b25 b23
        if s_22_12 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #55s : i
        let s_23_0: i128 = 55;
        // D s_23_1: read-var vaddr:u64
        let s_23_1: u64 = fn_state.vaddr;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 64u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #4s : i
        let s_23_5: i128 = 4;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_0 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_0)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 5u16);
        // C s_23_10: const #31u : u8
        let s_23_10: u8 = 31;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 5u16);
        // D s_23_12: cmp-eq s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) == (s_23_11));
        // D s_23_13: write-var gs#15844 <= s_23_12
        fn_state.gs_15844 = s_23_12;
        // N s_23_14: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#15844:u8
        let s_24_0: bool = fn_state.gs_15844;
        // D s_24_1: write-var gs#15845 <= s_24_0
        fn_state.gs_15845 = s_24_0;
        // N s_24_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#15844 <= s_25_0
        fn_state.gs_15844 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #16975u : u32
        let s_27_0: u32 = 16975;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: read-var vaddr:u64
        let s_27_2: u64 = fn_state.vaddr;
        // C s_27_3: const #0u : u8
        let s_27_3: bool = false;
        // D s_27_4: call EffectiveTBI(s_27_2, s_27_3, s_27_1)
        let s_27_4: bool = EffectiveTBI(state, tracer, s_27_2, s_27_3, s_27_1);
        // D s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // C s_27_6: const #0u : u8
        let s_27_6: bool = false;
        // C s_27_7: cast zx s_27_6 -> bv
        let s_27_7: Bits = Bits::new(s_27_6 as u128, 1u16);
        // D s_27_8: cmp-eq s_27_5 s_27_7
        let s_27_8: bool = ((s_27_5) == (s_27_7));
        // D s_27_9: write-var gs#15839 <= s_27_8
        fn_state.gs_15839 = s_27_8;
        // N s_27_10: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b15
        return block_15(state, tracer, fn_state);
    }
}
