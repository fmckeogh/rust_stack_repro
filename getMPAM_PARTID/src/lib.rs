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
use u_get_MPAMSM_EL1_Type_PARTID_D::*;
use u_get_MPAM1_EL1_Type_PARTID_D::*;
use u_get_MPAM2_EL2_Type_PARTID_I::*;
use MPAM3_EL3_read::*;
use u_get_MPAM2_EL2_Type_PARTID_D::*;
use u__UNKNOWN_PARTIDtype::*;
use u_get_MPAM1_EL1_Type_PARTID_I::*;
use u_get_MPAM0_EL1_Type_PARTID_I::*;
use MPAM1_EL1_read::*;
use Zeros::*;
use EL2Enabled::*;
use u_get_MPAM0_EL1_Type_PARTID_D::*;
use u_get_MPAM3_EL3_Type_PARTID_I::*;
use u_get_MPAM3_EL3_Type_PARTID_D::*;
use common::*;
pub fn getMPAM_PARTID<T: Tracer>(
    state: &mut State,
    tracer: &T,
    MPAMn: u8,
    InD: bool,
    InSM: bool,
) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        el2avail: bool,
        return_value: u16,
        partid: u16,
        MPAMn: u8,
        InD: bool,
        InSM: bool,
    }
    let fn_state = FunctionState {
        MPAMn,
        InD,
        InSM,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call EL2Enabled(s_0_0)
        let s_0_1: bool = EL2Enabled(state, tracer, s_0_0);
        // D s_0_2: write-var el2avail <= s_0_1
        fn_state.el2avail = s_0_1;
        // D s_0_3: read-var InSM:u8
        let s_0_3: bool = fn_state.InSM;
        // N s_0_4: branch s_0_3 b30 b1
        if s_0_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_1_0: read-var InD:u8
        let s_1_0: bool = fn_state.InD;
        // N s_1_1: branch s_1_0 b17 b2
        if s_1_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_2_0: read-var MPAMn:u8
        let s_2_0: u8 = fn_state.MPAMn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: not s_2_4
        let s_2_5: bool = !s_2_4;
        // N s_2_6: branch s_2_5 b7 b3
        if s_2_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call MPAM3_EL3_read(s_3_0)
        let s_3_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_3_0);
        // S s_3_2: call _get_MPAM3_EL3_Type_PARTID_D(s_3_1)
        let s_3_2: u16 = u_get_MPAM3_EL3_Type_PARTID_D(state, tracer, s_3_1);
        // D s_3_3: write-var partid <= s_3_2
        fn_state.partid = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_5_0: read-var partid:u16
        let s_5_0: u16 = fn_state.partid;
        // D s_5_1: write-var return_value <= s_5_0
        fn_state.return_value = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_6_0: read-var return_value:u16
        let s_6_0: u16 = fn_state.return_value;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_7_0: read-var MPAMn:u8
        let s_7_0: u8 = fn_state.MPAMn;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b12 b8
        if s_7_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_8_0: read-var el2avail:u8
        let s_8_0: bool = fn_state.el2avail;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_9_0: const #16s : i
        let s_9_0: i128 = 16;
        // S s_9_1: call Zeros(s_9_0)
        let s_9_1: Bits = Zeros(state, tracer, s_9_0);
        // S s_9_2: cast reint s_9_1 -> u16
        let s_9_2: u16 = (s_9_1.value() as u16);
        // D s_9_3: write-var partid <= s_9_2
        fn_state.partid = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // N s_10_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_11_0: const #90504u : u32
        let s_11_0: u32 = 90504;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_MPAM2_EL2_Type_PARTID_D(s_11_1)
        let s_11_2: u16 = u_get_MPAM2_EL2_Type_PARTID_D(state, tracer, s_11_1);
        // D s_11_3: write-var partid <= s_11_2
        fn_state.partid = s_11_2;
        // N s_11_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_12_0: read-var MPAMn:u8
        let s_12_0: u8 = fn_state.MPAMn;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call MPAM1_EL1_read(s_13_0)
        let s_13_1: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_13_0);
        // S s_13_2: call _get_MPAM1_EL1_Type_PARTID_D(s_13_1)
        let s_13_2: u16 = u_get_MPAM1_EL1_Type_PARTID_D(state, tracer, s_13_1);
        // D s_13_3: write-var partid <= s_13_2
        fn_state.partid = s_13_2;
        // N s_13_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_14_0: read-var MPAMn:u8
        let s_14_0: u8 = fn_state.MPAMn;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #0u : u8
        let s_14_2: u8 = 0;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b16 b15
        if s_14_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_15_0: const #16624u : u32
        let s_15_0: u32 = 16624;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_MPAM0_EL1_Type_PARTID_D(s_15_1)
        let s_15_2: u16 = u_get_MPAM0_EL1_Type_PARTID_D(state, tracer, s_15_1);
        // D s_15_3: write-var partid <= s_15_2
        fn_state.partid = s_15_2;
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call __UNKNOWN_PARTIDtype(s_16_0)
        let s_16_1: u16 = u__UNKNOWN_PARTIDtype(state, tracer, s_16_0);
        // D s_16_2: write-var partid <= s_16_1
        fn_state.partid = s_16_1;
        // N s_16_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_17_0: read-var MPAMn:u8
        let s_17_0: u8 = fn_state.MPAMn;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #3u : u8
        let s_17_2: u8 = 3;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b20 b18
        if s_17_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call MPAM3_EL3_read(s_18_0)
        let s_18_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_18_0);
        // S s_18_2: call _get_MPAM3_EL3_Type_PARTID_I(s_18_1)
        let s_18_2: u16 = u_get_MPAM3_EL3_Type_PARTID_I(state, tracer, s_18_1);
        // D s_18_3: write-var partid <= s_18_2
        fn_state.partid = s_18_2;
        // N s_18_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // N s_19_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_20_0: read-var MPAMn:u8
        let s_20_0: u8 = fn_state.MPAMn;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 2u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b25 b21
        if s_20_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_21_0: read-var el2avail:u8
        let s_21_0: bool = fn_state.el2avail;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_22_0: const #16s : i
        let s_22_0: i128 = 16;
        // S s_22_1: call Zeros(s_22_0)
        let s_22_1: Bits = Zeros(state, tracer, s_22_0);
        // S s_22_2: cast reint s_22_1 -> u16
        let s_22_2: u16 = (s_22_1.value() as u16);
        // D s_22_3: write-var partid <= s_22_2
        fn_state.partid = s_22_2;
        // N s_22_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // N s_23_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_24_0: const #90504u : u32
        let s_24_0: u32 = 90504;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_MPAM2_EL2_Type_PARTID_I(s_24_1)
        let s_24_2: u16 = u_get_MPAM2_EL2_Type_PARTID_I(state, tracer, s_24_1);
        // D s_24_3: write-var partid <= s_24_2
        fn_state.partid = s_24_2;
        // N s_24_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_25_0: read-var MPAMn:u8
        let s_25_0: u8 = fn_state.MPAMn;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 2u16);
        // C s_25_2: const #1u : u8
        let s_25_2: u8 = 1;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 2u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b27 b26
        if s_25_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call MPAM1_EL1_read(s_26_0)
        let s_26_1: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_26_0);
        // S s_26_2: call _get_MPAM1_EL1_Type_PARTID_I(s_26_1)
        let s_26_2: u16 = u_get_MPAM1_EL1_Type_PARTID_I(state, tracer, s_26_1);
        // D s_26_3: write-var partid <= s_26_2
        fn_state.partid = s_26_2;
        // N s_26_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_27_0: read-var MPAMn:u8
        let s_27_0: u8 = fn_state.MPAMn;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #0u : u8
        let s_27_2: u8 = 0;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 2u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b29 b28
        if s_27_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_28_0: const #16624u : u32
        let s_28_0: u32 = 16624;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_MPAM0_EL1_Type_PARTID_I(s_28_1)
        let s_28_2: u16 = u_get_MPAM0_EL1_Type_PARTID_I(state, tracer, s_28_1);
        // D s_28_3: write-var partid <= s_28_2
        fn_state.partid = s_28_2;
        // N s_28_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call __UNKNOWN_PARTIDtype(s_29_0)
        let s_29_1: u16 = u__UNKNOWN_PARTIDtype(state, tracer, s_29_0);
        // D s_29_2: write-var partid <= s_29_1
        fn_state.partid = s_29_1;
        // N s_29_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_30_0: const #102344u : u32
        let s_30_0: u32 = 102344;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_MPAMSM_EL1_Type_PARTID_D(s_30_1)
        let s_30_2: u16 = u_get_MPAMSM_EL1_Type_PARTID_D(state, tracer, s_30_1);
        // D s_30_3: write-var partid <= s_30_2
        fn_state.partid = s_30_2;
        // D s_30_4: read-var partid:u16
        let s_30_4: u16 = fn_state.partid;
        // D s_30_5: write-var return_value <= s_30_4
        fn_state.return_value = s_30_4;
        // N s_30_6: jump b6
        return block_6(state, tracer, fn_state);
    }
}
